use syn::spanned::Spanned;

use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = rules::constructor_naming::ConstructorNaming::CODE;

pub fn check(
    _self: &rules::constructor_naming::ConstructorNaming,
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
                if sig.receiver().is_some()
                    || sig.ident == "new"
                    || rules::constructor_naming::ConstructorNaming::takes_self_typed_argument(
                        sig, &self_name,
                    )
                {
                    continue;
                }
                let Some(shape) = rules::constructor_naming::ConstructorNaming::constructed_shape(
                    &sig.output,
                    &self_name,
                ) else {
                    continue;
                };
                if let Some(message) =
                    rules::constructor_naming::ConstructorNaming::advice(sig, &self_name, shape)
                {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::constructor_naming::ConstructorNaming;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("reading.rs"),
            syn::parse_str(
                "pub struct Reading;
                impl Reading {
                    pub fn at(sensor: &str, celsius: i32) -> Self {}
                    pub fn from_parts(sensor: &str, celsius: i32) -> Self {}
                    pub fn try_from_parts(sensor: &str, celsius: i32) -> Self {}
                    pub fn from_text(text: &str, base: u8) -> Result<Reading, Error> {}
                    pub fn try_from_text(text: &str, base: u8) -> Option<Self> {}
                    pub fn new(a: u8, b: u8) -> Self {}
                    pub fn single(a: u8) -> Self {}
                    pub fn sensor(&self, a: u8) -> Self {}
                    pub fn merge(current: Option<Self>, incoming: Reading) -> Self {}
                    pub fn copy(other: &Reading) -> Self {}
                }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ConstructorNaming, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 3, "{messages:?}");
        assert!(messages[0].contains("e.g. `from_at`"));
        assert!(messages[1].contains("e.g. `from_parts`"));
        assert!(messages[2].contains("e.g. `try_from_text`"));
    }
}
