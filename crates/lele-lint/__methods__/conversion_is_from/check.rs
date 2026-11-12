use std::collections::HashMap;
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
        let absolute = project.absolute_src_path(rel_path);
        let mut counts: HashMap<(String, String), usize> = HashMap::new();
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
                    || rules::conversion_is_from::ConversionIsFrom::is_exempt_from_conversion(sig)
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
                if shape != "Self" && shape != "Result" {
                    continue;
                }
                let Some(argument) =
                    rules::conversion_is_from::ConversionIsFrom::single_argument_type(sig)
                else {
                    continue;
                };
                let count = counts.entry((self_name.clone(), argument)).or_default();
                *count = count.saturating_add(1);
            }
        }
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
                    || rules::conversion_is_from::ConversionIsFrom::is_exempt_from_conversion(sig)
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
                if let Some(argument) =
                    rules::conversion_is_from::ConversionIsFrom::single_argument_type(sig)
                {
                    let count = counts
                        .get(&(self_name.clone(), argument))
                        .copied()
                        .unwrap_or(0);
                    if (shape == "Self" || shape == "Result") && count >= 2 {
                        continue;
                    }
                }
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
                    pub fn parse(count: u64) -> Result<Reading, Error> {}
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
        assert!(messages[1].contains("impl TryFrom<u64> for Reading"));
    }

    #[test]
    fn test_usage_async_const_and_generic_builders_are_exempt() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("session.rs"),
            syn::parse_str(
                "pub struct Session;
                impl Session {
                    pub async fn connect(url: &str) -> std::io::Result<Self> {}
                    pub const fn sized(size: usize) -> Self {}
                    pub fn named(name: impl Into<String>) -> Self {}
                    pub fn tagged<T: Into<String>>(tag: T) -> Self {}
                }",
            )
            .unwrap(),
        );
        let violations = check(&ConversionIsFrom, &project);
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn test_usage_plain_builder_with_primitive_argument_advises_newtype() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("deck.rs"),
            syn::parse_str(
                "pub struct Deck;
                impl Deck {
                    pub fn sized(size: usize) -> Self {}
                }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ConversionIsFrom, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 1, "{messages:?}");
        assert!(
            messages[0].contains("name the argument first"),
            "{messages:?}"
        );
    }

    #[test]
    fn test_usage_named_constructors_sharing_type_are_exempt() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("duration.rs"),
            syn::parse_str(
                "pub struct Duration;
                impl Duration {
                    pub fn from_secs(secs: u64) -> Self {}
                    pub fn from_millis(millis: u64) -> Self {}
                }",
            )
            .unwrap(),
        );
        let violations = check(&ConversionIsFrom, &project);
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn test_usage_named_constructors_across_impl_blocks_are_exempt() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("duration.rs"),
            syn::parse_str(
                "pub struct Duration;
                impl Duration { pub fn from_secs(secs: u64) -> Self {} }
                impl Duration { pub fn from_millis(millis: u64) -> Self {} }",
            )
            .unwrap(),
        );
        let violations = check(&ConversionIsFrom, &project);
        assert!(violations.is_empty(), "{violations:?}");
    }

    #[test]
    fn test_usage_different_argument_types_are_both_reported() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("timeout.rs"),
            syn::parse_str(
                "pub struct Timeout;
                impl Timeout {
                    pub fn from_secs(secs: u64) -> Self {}
                    pub fn from_name(name: &str) -> Self {}
                }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ConversionIsFrom, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 2, "{messages:?}");
        assert!(messages[0].contains("Timeout"));
        assert!(messages[1].contains("Timeout"));
    }
}
