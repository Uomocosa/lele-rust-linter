use crate::rules;
use crate::ItemMarker;
use crate::Origin;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::methods_are_delegates::MethodsAreDelegates,
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
            let message = match item {
                syn::Item::Use(_) => None,
                syn::Item::Const(item) if matches!(item.vis, syn::Visibility::Inherited) => None,
                syn::Item::Mod(item)
                    if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) =>
                {
                    None
                }
                syn::Item::Fn(item)
                    if item.sig.ident == stem && matches!(item.vis, syn::Visibility::Public(_)) =>
                {
                    None
                }
                syn::Item::Fn(item) => Some(format!(
                    "`{}` is a helper in a method file; only `pub fn {stem}` lives here — declare it as a `pub(crate)` method or associated fn of the type in `src/` and call it through the type",
                    item.sig.ident
                )),
                syn::Item::Impl(_) => Some(
                    "`impl` block in a method file; impls live next to their type in `src/`, bodies in `methods/<type>/<method>.rs`".to_string(),
                ),
                syn::Item::Struct(syn::ItemStruct { ident, .. })
                | syn::Item::Enum(syn::ItemEnum { ident, .. })
                | syn::Item::Trait(syn::ItemTrait { ident, .. })
                | syn::Item::Type(syn::ItemType { ident, .. }) => Some(format!(
                    "type `{ident}` in a method file; types live in `src/` (`pub(crate)` for internal machinery)"
                )),
                _ => Some(format!(
                    "only `use`, private `const`, `pub fn {stem}` and `#[cfg(test)] mod tests` may appear in a method file"
                )),
            };
            if let Some(message) = message {
                violations.push(RuleViolation::from_file(
                    project.absolute_path(Origin::Methods, rel_path),
                    rules::methods_are_delegates::MethodsAreDelegates::CODE,
                    message,
                ));
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::methods_are_delegates::MethodsAreDelegates;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("player/heal.rs"),
            syn::parse_str(
                "use crate::player; const CAP: u32 = 100; pub fn heal() {} fn capped() {} pub(crate) fn shared() {} struct Hit; impl Hit {} #[cfg(test)] mod tests {}",
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
        assert_eq!(messages.len(), 4);
        assert!(messages.iter().any(|m| m.starts_with("`capped`")));
        assert!(messages.iter().any(|m| m.starts_with("`shared`")));
        assert!(messages.iter().any(|m| m.starts_with("type `Hit`")));
        assert!(messages.iter().any(|m| m.starts_with("`impl` block")));
    }
}
