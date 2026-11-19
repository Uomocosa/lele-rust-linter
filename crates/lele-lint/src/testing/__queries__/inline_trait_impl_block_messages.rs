use std::path::Path;

use crate::rules;
use crate::InlineTraitMethodsEntry;
use crate::Project;

pub(crate) fn inline_trait_impl_block_messages(
    src: &str,
    trait_path: &str,
    methods: &[&str],
) -> Vec<String> {
    let project = Project {
        inline_trait_methods: vec![InlineTraitMethodsEntry {
            trait_path: trait_path.to_string(),
            methods: methods.iter().map(|method| (*method).to_string()).collect(),
            reason: "listed by the test".to_string(),
        }],
        ..Project::default()
    };
    let file: syn::File = syn::parse_str(src).unwrap();
    let mut violations = Vec::new();
    for item in &file.items {
        if let syn::Item::Impl(impl_block) = item {
            violations.extend(
                rules::inline_trait_methods::InlineTraitMethods::impl_block_violations(
                    &project,
                    Path::new("foo.rs"),
                    &file,
                    impl_block,
                ),
            );
        }
    }
    violations
        .into_iter()
        .map(|violation| violation.message)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::inline_trait_impl_block_messages;

    #[test]
    fn test_usage() {
        let inline = "pub struct Foo;\nimpl Plugin for Foo { fn build(&self) { self.run(); } }\n";
        assert_eq!(
            inline_trait_impl_block_messages(inline, "Plugin", &["build"]),
            Vec::<String>::new()
        );
    }
}
