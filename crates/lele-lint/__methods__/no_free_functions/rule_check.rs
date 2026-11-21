use std::collections::BTreeSet;
use syn::spanned::Spanned;

use crate::config;
use crate::rules;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = rules::no_free_functions::NoFreeFunctions::CODE;

pub fn rule_check(
    _self: &rules::no_free_functions::NoFreeFunctions,
    project: &Project,
) -> Vec<RuleViolation> {
    let local_types = rules::no_free_functions::NoFreeFunctions::local_type_names(project);
    let mut violations = Vec::new();
    let mut matched_entry_points = BTreeSet::new();

    for (rel_path, file) in &project.parsed_files {
        let exempt = rules::no_free_functions::NoFreeFunctions::is_exempt_file(rel_path, project);
        let absolute = project.absolute_src_path(rel_path);
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
                entry.function == name
                    && Project::same_file(&project.config_path(&entry.file), &absolute)
            });
            if let Some(index) = entry_point {
                matched_entry_points.insert(index);
                continue;
            }
            if exempt {
                continue;
            }
            violations.push(RuleViolation::from_site(
                absolute.clone(),
                func.sig.fn_token.span().start().line,
                0,
                CODE,
                rules::no_free_functions::NoFreeFunctions::owner_hint(
                    project,
                    &func.sig,
                    &local_types,
                ),
            ));
        }
    }

    for (index, entry) in project.entry_points.iter().enumerate() {
        let path = project.config_path(&entry.file);
        if !path.is_file() {
            violations.push(RuleViolation::from_file(
                project.config_base().join(config::CONFIG_FILENAME),
                CODE,
                format!(
                    "stale [[lele.project.entry_points]] entry: file `{}` does not exist — delete the entry",
                    entry.file.display()
                ),
            ));
            continue;
        }
        if matched_entry_points.contains(&index) || !project.is_in_crate(&path) {
            continue;
        }
        violations.push(RuleViolation::from_file(
            path,
            CODE,
            format!(
                "stale [[lele.project.entry_points]] entry: no free `pub fn {}` in `{}` — delete the entry",
                entry.function,
                entry.file.display()
            ),
        ));
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::rule_check;
    use crate::rules::no_free_functions::NoFreeFunctions;
    use crate::testing;
    use crate::EntryPoint;

    #[test]
    fn test_usage() {
        let project = testing::parsed_project(
            testing::crate_project(),
            &[
                ("player/player.rs", "pub struct Player;"),
                (
                    "player/heal.rs",
                    "pub fn heal(player: &mut Player, amount: u32) {}",
                ),
            ],
        );
        let violations = rule_check(&NoFreeFunctions, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("impl Player"));
        assert!(violations[0].message.contains("__methods__/player/heal.rs"));
    }

    #[test]
    fn test_return_type_and_two_candidates() {
        let project = testing::parsed_project(
            testing::crate_project(),
            &[
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
            ],
        );
        let messages: Vec<String> = rule_check(&NoFreeFunctions, &project)
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
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/run.rs"), "pub fn run() {}").unwrap();
        let mut project = testing::parsed_project(
            testing::dunder_project(),
            &[
                ("game/__bevy_systems__/tick.rs", "pub fn tick() {}"),
                ("main.rs", "pub fn run() {}"),
                (
                    "run.rs",
                    "pub fn run() {} fn private() {} #[cfg(test)] pub fn t() {}",
                ),
            ],
        );
        project.root = dir.path().to_path_buf();
        project.src_dir = dir.path().join("src");
        project.entry_points.push(EntryPoint {
            file: PathBuf::from("src/run.rs"),
            function: "run".to_string(),
            reason: "task".to_string(),
        });
        assert!(rule_check(&NoFreeFunctions, &project).is_empty());
    }

    #[test]
    fn test_config_relative_entry_matches_in_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let project = testing::workspace_project(dir.path(), "crates/a/src/run.rs");
        assert!(rule_check(&NoFreeFunctions, &project).is_empty());
    }

    #[test]
    fn test_entry_for_other_crate_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let project = testing::workspace_project(dir.path(), "crates/b/src/run.rs");
        std::fs::create_dir_all(dir.path().join("crates/b/src")).unwrap();
        std::fs::write(dir.path().join("crates/b/src/run.rs"), "pub fn run() {}").unwrap();
        let messages: Vec<String> = rule_check(&NoFreeFunctions, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 1, "{messages:?}");
        assert!(!messages[0].contains("stale"));
    }

    #[test]
    fn test_missing_entry_file_is_stale() {
        let dir = tempfile::tempdir().unwrap();
        let project = testing::workspace_project(dir.path(), "crates/a/src/gone.rs");
        let messages: Vec<String> = rule_check(&NoFreeFunctions, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert!(messages
            .iter()
            .any(|m| m.contains("stale") && m.contains("does not exist")));
    }

    #[test]
    fn test_unmatched_in_crate_entry_is_stale() {
        let dir = tempfile::tempdir().unwrap();
        let mut project = testing::workspace_project(dir.path(), "crates/a/src/run.rs");
        project.entry_points[0].function = "missing_fn".to_string();
        let messages: Vec<String> = rule_check(&NoFreeFunctions, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert!(messages.iter().any(|m| m.contains("stale")));
    }

    #[test]
    fn test_binaries_are_not_exempt() {
        let project = testing::parsed_project(
            testing::crate_project(),
            &[("bin/tool.rs", "pub fn helper() {}")],
        );
        let violations = rule_check(&NoFreeFunctions, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("helper"));
    }
}
