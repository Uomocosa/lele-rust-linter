use syn::spanned::Spanned;

use crate::checkers;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = checkers::conversion_is_from::ConversionIsFrom::CODE;

pub fn check(
    _self: &checkers::conversion_is_from::ConversionIsFrom,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.parsed_files {
        let absolute = project.src_dir.join(rel_path);
        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };
            if impl_block.trait_.is_some()
                || impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            let Some(self_name) = ImplBlock(impl_block).self_type_last() else {
                continue;
            };
            for impl_item in &impl_block.items {
                let syn::ImplItem::Fn(method) = impl_item else {
                    continue;
                };
                let sig = &method.sig;
                if sig.receiver().is_some() || sig.ident == "new" {
                    continue;
                }
                let Some(shape) = constructed_shape(&sig.output, &self_name) else {
                    continue;
                };
                if let Some(message) = advice(sig, &self_name, shape) {
                    violations.push(RuleViolation::from_site(
                        absolute.clone(),
                        sig.fn_token.span().start().line,
                        0,
                        CODE,
                        message,
                    ));
                }
            }
        }
    }
    violations
}

// needed helper: one argument building `Self` or `Result<Self, _>` is a `From`/`TryFrom` conversion
fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {
    let [syn::FnArg::Typed(argument)] = sig.inputs.iter().collect::<Vec<_>>()[..] else {
        return None;
    };
    let source = quote::ToTokens::to_token_stream(&*argument.ty).to_string();
    let name = &sig.ident;
    match shape {
        "Self" => Some(format!(
            "`{self_name}::{name}` converts `{source}` into `{self_name}` — write it as `impl From<{source}> for {self_name}` and call `{self_name}::from(..)` / `.into()`"
        )),
        "Result" => Some(format!(
            "`{self_name}::{name}` is a fallible conversion from `{source}` — write it as `impl TryFrom<{source}> for {self_name}` and call `{self_name}::try_from(..)` / `.try_into()`"
        )),
        _ => None,
    }
}

// needed helper: `Self`, `Result<Self, _>` or `Option<Self>` — the wrapper name, or `Self`
fn constructed_shape(output: &syn::ReturnType, self_name: &str) -> Option<&'static str> {
    fn is_self(ty: &syn::Type, self_name: &str) -> bool {
        matches!(ty, syn::Type::Path(path)
        if path.qself.is_none()
            && path.path.segments.len() == 1
            && path.path.segments.last().is_some_and(|segment| {
                segment.arguments.is_none()
                    && (segment.ident == "Self" || segment.ident == self_name)
            }))
    }

    let syn::ReturnType::Type(_, ty) = output else {
        return None;
    };
    if is_self(ty, self_name) {
        return Some("Self");
    }
    let syn::Type::Path(path) = &**ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let wrapper = match segment.ident.to_string().as_str() {
        "Result" => "Result",
        "Option" => "Option",
        _ => return None,
    };
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        syn::GenericArgument::Type(inner) if is_self(inner, self_name) => Some(wrapper),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::conversion_is_from::ConversionIsFrom;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("reading.rs"),
            syn::parse_str(
                "pub struct Reading;
                impl Reading {
                    pub fn from_sensor(sensor: &str) -> Self {}
                    pub fn parse(text: &str) -> Result<Reading, Error> {}
                    pub fn find(text: &str) -> Option<Self> {}
                    pub fn new(sensor: &str) -> Self {}
                    pub fn pair(a: u8, b: u8) -> Self {}
                    pub fn empty() -> Self {}
                    pub fn boxed(a: u8) -> Box<Self> {}
                    pub fn sensor(&self, a: u8) -> Self {}
                }
                impl From<u8> for Reading { fn from(a: u8) -> Self {} }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ConversionIsFrom, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 2, "{messages:?}");
        assert!(messages[0].contains("impl From<& str> for Reading"));
        assert!(messages[1].contains("impl TryFrom<& str> for Reading"));
    }
}
