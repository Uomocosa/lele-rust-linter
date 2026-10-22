use std::path::Path;

use crate::checkers;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = checkers::methods_are_delegates::MethodsAreDelegates::CODE;

pub fn check(
    _self: &checkers::methods_are_delegates::MethodsAreDelegates,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.methods_parsed_files {
        if rel_path.file_name().is_some_and(|name| name == "mod.rs") {
            continue;
        }
        let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        for item in &file.items {
            let Some((ident, visibility)) = named_visibility(item) else {
                continue;
            };
            let is_delegate = matches!(item, syn::Item::Fn(_))
                && ident == stem
                && matches!(visibility, syn::Visibility::Public(_));
            if is_delegate || matches!(visibility, syn::Visibility::Inherited) {
                continue;
            }
            violations.push(violation(project, rel_path, stem, &ident));
        }
    }
    violations
}

// needed helper: the name and visibility of every item that can be exported
fn named_visibility(item: &syn::Item) -> Option<(String, &syn::Visibility)> {
    let (ident, visibility, attrs) = match item {
        syn::Item::Fn(item) => (&item.sig.ident, &item.vis, &item.attrs),
        syn::Item::Struct(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Enum(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Trait(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Const(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Static(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Type(item) => (&item.ident, &item.vis, &item.attrs),
        syn::Item::Mod(item) => (&item.ident, &item.vis, &item.attrs),
        _ => return None,
    };
    if attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
        return None;
    }
    Some((ident.to_string(), visibility))
}

// needed helper: diagnostic anchored to the methods-relative path
fn violation(project: &Project, rel_path: &Path, stem: &str, ident: &str) -> RuleViolation {
    let file = project
        .methods_dir
        .as_ref()
        .map_or_else(|| rel_path.to_path_buf(), |dir| dir.join(rel_path));
    RuleViolation::from_file(
        file,
        CODE,
        format!(
            "`{ident}` is exported from a method file; only `pub fn {stem}` may be public here — make `{ident}` private with `// needed helper: <why>`, or give it its own `#[atomic_delegates]` shell and file"
        ),
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::methods_are_delegates::MethodsAreDelegates;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("player/heal.rs"),
            syn::parse_str(
                "pub fn heal() {} fn capped() {} pub(crate) fn shared() {} pub struct Leak; #[cfg(test)] pub mod tests {}",
            )
            .unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("player/mod.rs"),
            syn::parse_str("mod heal; pub use heal::heal;").unwrap(),
        );
        let messages: Vec<String> = check(&MethodsAreDelegates, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 2);
        assert!(messages.iter().any(|m| m.starts_with("`shared`")));
        assert!(messages.iter().any(|m| m.starts_with("`Leak`")));
    }
}
