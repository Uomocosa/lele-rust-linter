use syn::spanned::Spanned;

use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = rules::conversion_is_from::ConversionIsFrom::CODE;

pub fn check(
    _self: &rules::conversion_is_from::ConversionIsFrom,
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
                    || rules::conversion_is_from::ConversionIsFrom::takes_self_typed_argument(
                        sig, &self_name,
                    )
                {
                    continue;
                }
                let Some(shape) = rules::conversion_is_from::ConversionIsFrom::constructed_shape(
                    &sig.output,
                    &self_name,
                ) else {
                    continue;
                };
                if let Some(message) =
                    rules::conversion_is_from::ConversionIsFrom::advice(sig, &self_name, shape)
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
    use crate::rules::conversion_is_from::ConversionIsFrom;
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
                    pub fn merge(current: Option<Self>, incoming: Reading) -> Self {}
                    pub fn copy(other: &Reading) -> Self {}
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
