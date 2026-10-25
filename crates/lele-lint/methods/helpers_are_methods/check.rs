use syn::spanned::Spanned;

use crate::rules;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = rules::helpers_are_methods::HelpersAreMethods::CODE;

pub fn check(
    _self: &rules::helpers_are_methods::HelpersAreMethods,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.parsed_files {
        if rel_path.components().any(|c| c.as_os_str() == "tests") {
            continue;
        }
        let is_bin_root = rel_path
            .parent()
            .is_some_and(|dir| dir == std::path::Path::new("bin"));
        let is_main_file =
            is_bin_root || rel_path.file_name().is_some_and(|name| name == "main.rs");
        let absolute = project.src_dir.join(rel_path);
        for item in &file.items {
            match item {
                syn::Item::Fn(func)
                    if matches!(func.vis, syn::Visibility::Inherited)
                        && !func.attrs.iter().any(|attr| {
                            ItemMarker(attr).is_test() || ItemMarker(attr).is_cfg_test()
                        })
                        && !(is_main_file && func.sig.ident == "main") =>
                {
                    violations.push(RuleViolation::from_site(
                        absolute.clone(),
                        func.sig.fn_token.span().start().line,
                        0,
                        CODE,
                        format!(
                            "private free fn `{}` is a helper — make it a `pub(crate)` method or associated fn of the type it serves (an `#[atomic_delegates]` shell, or inline under `#[rustfmt::skip]`)",
                            func.sig.ident
                        ),
                    ));
                }
                syn::Item::Impl(block)
                    if block.trait_.is_none()
                        && !block
                            .attrs
                            .iter()
                            .any(|attr| ItemMarker(attr).is_cfg_test()) =>
                {
                    for impl_item in &block.items {
                        let syn::ImplItem::Fn(method) = impl_item else {
                            continue;
                        };
                        if matches!(method.vis, syn::Visibility::Inherited)
                            && !method
                                .attrs
                                .iter()
                                .any(|attr| ItemMarker(attr).is_cfg_test())
                        {
                            violations.push(RuleViolation::from_site(
                                absolute.clone(),
                                method.sig.fn_token.span().start().line,
                                0,
                                CODE,
                                format!(
                                    "private method `{0}` — a helper is `pub(crate)`: write `pub(crate) fn {0}`",
                                    method.sig.ident
                                ),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::helpers_are_methods::HelpersAreMethods;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("round.rs"),
            syn::parse_str(
                "pub struct Round; impl Round { pub fn score(&self) {} fn hidden(&self) {} pub(crate) fn shared() {} } impl Default for Round { fn default() -> Self { Self } } fn helper() {} #[test] fn test_a() {}",
            )
            .unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("main.rs"),
            syn::parse_str("fn main() {}").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("bin/new_game_id.rs"),
            syn::parse_str("fn main() {} fn bin_helper() {}").unwrap(),
        );
        let messages: Vec<String> = check(&HelpersAreMethods, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 3);
        assert!(messages.iter().any(|m| m.contains("`bin_helper`")));
        assert!(messages.iter().any(|m| m.contains("`hidden`")));
        assert!(messages.iter().any(|m| m.contains("`helper`")));
    }
}
