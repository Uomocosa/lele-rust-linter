use std::path::Path;

use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;
use lele_snake_case::traits::ToLeleSnakeCase;

pub fn impl_block_violations(
    project: &Project,
    rel_path: &Path,
    file: &syn::File,
    impl_block: &syn::ItemImpl,
) -> Vec<RuleViolation> {
    let listed =
        rules::inline_trait_methods::InlineTraitMethods::listed_methods(project, impl_block);
    if listed.is_empty() {
        return Vec::new();
    }
    let code = rules::inline_trait_methods::InlineTraitMethods::CODE;
    let trait_name = ImplBlock(impl_block).trait_ident().unwrap_or_default();
    let self_type = ImplBlock(impl_block).self_type_last().unwrap_or_default();
    let block_is_atomic = impl_block
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).is_atomic_delegates());
    let file_path = project.absolute_src_path(rel_path);
    let mut violations = Vec::new();
    for item in &impl_block.items {
        let syn::ImplItem::Fn(method) = item else {
            continue;
        };
        let name = method.sig.ident.to_string();
        if !listed.contains(&name) {
            continue;
        }
        let location = format!("`{trait_name}::{name}` for `{self_type}`");
        let body_file = format!(
            "{}/{}/{}.rs",
            project.methods_root_name(),
            self_type.to_snake_case(),
            ImplBlock(impl_block).delegate_body_name(&method.sig.ident)
        );
        let annotated = block_is_atomic
            || method
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_atomic_delegate());
        let message = if annotated {
            format!(
                "{location} must stay inline (listed in [[lele.lint.inline_trait_methods]]) — remove its `#[atomic_delegate]`/`#[atomic_delegates]` annotation, write the body in the trait impl and delete `{body_file}`"
            )
        } else if method.block.stmts.is_empty() {
            format!(
                "{location} must stay inline (listed in [[lele.lint.inline_trait_methods]]) — an empty body is a delegate shell, so write the body in the trait impl"
            )
        } else if let Some(shell_file) =
            rules::inline_trait_methods::InlineTraitMethods::called_shell_file(
                project, file, impl_block, method,
            )
        {
            format!(
                "{location} must stay inline (listed in [[lele.lint.inline_trait_methods]]) — it only calls a delegate shell, so write the body in the trait impl and delete `{shell_file}`"
            )
        } else {
            continue;
        };
        violations.push(RuleViolation::from_file(file_path.clone(), code, message));
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::rules;
    use crate::InlineTraitMethodsEntry;
    use crate::Project;

    fn violations_for(src: &str, trait_path: &str, methods: &[&str]) -> Vec<String> {
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

    #[test]
    fn test_usage() {
        let shell_call = "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { pub fn build(&self) {} }\nimpl Plugin for Foo { fn build(&self) { self.build() } }\n";
        let messages = violations_for(shell_call, "Plugin", &["build"]);
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("must stay inline"));
        assert!(messages[0].contains("foo/build.rs"));
        let inline = "pub struct Foo;\nimpl Plugin for Foo { fn build(&self) { self.run(); } }\n";
        assert_eq!(
            violations_for(inline, "Plugin", &["build"]),
            Vec::<String>::new()
        );
        let other_trait = "pub struct Foo;\nimpl Other for Foo { fn build(&self) {} }\n";
        assert_eq!(
            violations_for(other_trait, "Plugin", &["build"]),
            Vec::<String>::new()
        );
        let empty_body = "pub struct Foo;\nimpl Plugin for Foo { fn build(&self) {} }\n";
        assert_eq!(violations_for(empty_body, "Plugin", &["build"]).len(), 1);
        let annotated = "pub struct Foo;\nimpl Plugin for Foo { #[atomic_delegate(Foo, Plugin)] fn build(&self) { self.run(); } }\n";
        assert_eq!(violations_for(annotated, "Plugin", &["build"]).len(), 1);
    }
}
