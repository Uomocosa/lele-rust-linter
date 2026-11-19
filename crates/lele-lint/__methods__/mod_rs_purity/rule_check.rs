use crate::rules;
use crate::Project;
use crate::RuleViolation;
use syn::spanned::Spanned;

pub fn rule_check(
    _self: &rules::mod_rs_purity::ModRsPurity,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let Some(file_name) = rel_path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if file_name != "mod.rs" {
            continue;
        }

        for item in &file.items {
            let Some(message) = rules::mod_rs_purity::ModRsPurity::violation_message(item) else {
                continue;
            };
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, rel_path),
                item.span().start().line,
                0,
                rules::mod_rs_purity::ModRsPurity::CODE,
                message,
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::testing;

    #[test]
    fn test_usage() {
        assert!(testing::mod_rs_violation_message("mod player;").is_none());
        assert!(testing::mod_rs_violation_message("pub use player::Player;").is_none());
        assert!(testing::mod_rs_violation_message("use crate::rule;").is_some());
    }

    #[test]
    fn test_usage_allows_mod_declarations() {
        assert!(testing::mod_rs_violation_message("mod player;").is_none());
        assert!(testing::mod_rs_violation_message("pub mod bevy_systems;").is_none());
        assert!(testing::mod_rs_violation_message("pub(crate) mod spawned_peer;").is_none());
    }

    #[test]
    fn test_usage_allows_pub_use_reexports() {
        assert!(testing::mod_rs_violation_message("pub use player::Player;").is_none());
        assert!(
            testing::mod_rs_violation_message("pub(crate) use spawned_peer::SpawnedPeer;")
                .is_none()
        );
        assert!(testing::mod_rs_violation_message("pub use constants::*;").is_none());
    }

    #[test]
    fn test_usage_flags_private_use() {
        let m = testing::mod_rs_violation_message("use crate::rule;").unwrap();
        assert!(m.contains("private `use` import"));
    }

    #[test]
    fn test_usage_flags_test_module() {
        let m = testing::mod_rs_violation_message("#[cfg(test)] mod tests { fn test_usage() {} }")
            .unwrap();
        assert!(m.contains("#[cfg(test)] mod"));
    }

    #[test]
    fn test_usage_flags_logic_items() {
        assert!(
            testing::mod_rs_violation_message("pub struct Score { pub value: u32 }")
                .unwrap()
                .contains("struct")
        );
        assert!(testing::mod_rs_violation_message("pub fn build_rules() {}")
            .unwrap()
            .contains("fn build_rules"));
        assert!(
            testing::mod_rs_violation_message("impl Player { fn tick(&self) {} }")
                .unwrap()
                .contains("impl")
        );
        assert!(
            testing::mod_rs_violation_message("pub const MAX: u32 = 10;")
                .unwrap()
                .contains("const")
        );
    }
}
