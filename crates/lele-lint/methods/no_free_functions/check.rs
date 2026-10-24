use std::collections::BTreeSet;
use syn::spanned::Spanned;

use crate::rules;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = rules::no_free_functions::NoFreeFunctions::CODE;

pub fn check(
    _self: &rules::no_free_functions::NoFreeFunctions,
    project: &Project,
) -> Vec<RuleViolation> {
    let local_types = rules::no_free_functions::NoFreeFunctions::local_type_names(project);
    let mut violations = Vec::new();
    let mut matched_entry_points = BTreeSet::new();

    for (rel_path, file) in &project.parsed_files {
        if rules::no_free_functions::NoFreeFunctions::is_exempt_file(rel_path, project) {
            continue;
        }
        let absolute = project.src_dir.join(rel_path);
        for item in &file.items {
            let syn::Item::Fn(func) = item else {
                continue;
            };
            if matches!(func.vis, syn::Visibility::Inherited)
                || func.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test())
                || func.attrs.iter().any(|attr| ItemMarker(attr).is_test())
            {
                continue;
            }
            let name = func.sig.ident.to_string();
            let entry_point = project.entry_points.iter().position(|entry| {
                entry.function == name && project.root.join(&entry.file) == absolute
            });
            if let Some(index) = entry_point {
                matched_entry_points.insert(index);
                continue;
            }
            violations.push(RuleViolation::from_site(
                absolute.clone(),
                func.sig.fn_token.span().start().line,
                0,
                CODE,
                rules::no_free_functions::NoFreeFunctions::owner_hint(&func.sig, &local_types),
            ));
        }
    }

    for (index, entry) in project.entry_points.iter().enumerate() {
        let path = project.root.join(&entry.file);
        if !matched_entry_points.contains(&index) && path.is_file() {
            violations.push(RuleViolation::from_file(
                path,
                CODE,
                format!(
                    "stale [[lele.lint.entry_points]] entry: no free `pub fn {}` in `{}` — delete the entry",
                    entry.function,
                    entry.file.display()
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::no_free_functions::NoFreeFunctions;
    use crate::EntryPoint;
    use crate::Project;

    fn project_with(files: &[(&str, &str)]) -> Project {
        let mut project = Project {
            root: PathBuf::from("/crate"),
            src_dir: PathBuf::from("/crate/src"),
            ..Project::default()
        };
        for (path, source) in files {
            project
                .parsed_files
                .insert(PathBuf::from(path), syn::parse_str(source).unwrap());
        }
        project
    }

    #[test]
    fn test_usage() {
        let project = project_with(&[
            ("player/player.rs", "pub struct Player;"),
            (
                "player/heal.rs",
                "pub fn heal(player: &mut Player, amount: u32) {}",
            ),
        ]);
        let violations = check(&NoFreeFunctions, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("impl Player"));
        assert!(violations[0].message.contains("methods/player/heal.rs"));
    }

    #[test]
    fn test_return_type_and_two_candidates() {
        let project = project_with(&[
            ("a.rs", "pub struct Client; pub struct Config;"),
            (
                "connect.rs",
                "pub fn connect(host: &str) -> Result<Client, E> {}",
            ),
            (
                "make.rs",
                "pub fn build(config: &Config) -> Option<Client> {}",
            ),
            ("split.rs", "pub fn split(bytes: &[u8]) -> Vec<u8> {}"),
        ]);
        let messages: Vec<String> = check(&NoFreeFunctions, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert!(messages
            .iter()
            .any(|m| m.contains("impl Client` (return type")));
        assert!(messages
            .iter()
            .any(|m| m.contains("`Config` (first argument) or `Client` (return type)")));
        assert!(messages.iter().any(|m| m.contains("newtype")));
    }

    #[test]
    fn test_exemptions_and_entry_points() {
        let mut project = project_with(&[
            ("game/bevy_systems/tick.rs", "pub fn tick() {}"),
            ("main.rs", "pub fn run() {}"),
            (
                "run.rs",
                "pub fn run() {} fn private() {} #[cfg(test)] pub fn t() {}",
            ),
        ]);
        project.entry_points.push(EntryPoint {
            file: PathBuf::from("src/run.rs"),
            function: "run".to_string(),
            reason: "task".to_string(),
        });
        assert!(check(&NoFreeFunctions, &project).is_empty());
    }

    #[test]
    fn test_binaries_are_not_exempt() {
        let project = project_with(&[("bin/tool.rs", "pub fn helper() {}")]);
        let violations = check(&NoFreeFunctions, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("helper"));
    }
}
