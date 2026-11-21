use std::path::Path;

use crate::Config;
use crate::Error;
use crate::LeleToml;

pub fn try_from_path(path: &Path) -> Result<Config, Error> {
    let content = std::fs::read_to_string(path).map_err(Error::Io)?;
    let parsed: LeleToml = toml::from_str(&content)?;
    Ok(Config(Some(parsed.lele)))
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use crate::Error;
    use crate::Project;
    use crate::Requirement;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        let config = super::try_from_path(&path).unwrap();
        assert!(config.as_ref().is_some());
    }

    #[test]
    fn config_try_from_accepts_empty_project_section() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        assert!(Config::try_from(path.as_path()).unwrap().as_ref().is_some());
    }

    #[test]
    fn test_usage_rejects_old_lint_section() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("lele.toml"), "[lele.lint]\n").unwrap();
        let error = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap_err();
        let err = std::error::Error::source(&error)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(err.contains("lint"), "{err}");
    }

    #[test]
    fn test_usage_parses_rule_exemptions() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[lele.project]\n\
             exclude = [\"target\"]\n\
             e2e_test_dirs = [\"crates/x/tests\"]\n\
             \n\
             [lele.rules.lint.no_allow_attributes]\n\
             level = \"deny\"\n\
             reason = \"example entries\"\n\
             [[lele.rules.lint.no_allow_attributes.exempt]]\n\
             allow = \"clippy::panic_in_result_fn\"\n\
             file = \"crates/x/src/lib.rs\"\n\
             reason = \"probe\"\n\
             \n\
             [[lele.rules.lint.no_collection_newtype.exempt]]\n\
             type = \"Deck\"\n\
             file = \"src/cards/deck.rs\"\n\
             reason = \"at most 52 cards\"\n\
             \n\
             [[lele.rules.lint.atomic_delegates.exempt]]\n\
             crate = \"crates/lele-snake-case\"\n\
             reason = \"cannot depend on the macros\"\n",
        )
        .unwrap();
        let config = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap();
        let section = config.as_ref().unwrap();
        assert_eq!(section.project.exclude.len(), 1);
        assert_eq!(section.project.e2e_test_dirs.len(), 1);
        let allow = section
            .rules
            .get("lint")
            .and_then(|namespace| namespace.get("no_allow_attributes"))
            .expect("no_allow_attributes table");
        assert_eq!(allow.exempt.len(), 1);
        let collection = section
            .rules
            .get("lint")
            .and_then(|namespace| namespace.get("no_collection_newtype"))
            .expect("no_collection_newtype table");
        assert_eq!(collection.exempt.len(), 1);
        let delegates = section
            .rules
            .get("lint")
            .and_then(|namespace| namespace.get("atomic_delegates"))
            .expect("atomic_delegates table");
        assert_eq!(delegates.exempt.len(), 1);
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        project.apply_layout(&config).unwrap();
        assert_eq!(project.clippy_allow_whitelist.len(), 1);
        assert_eq!(project.collection_wrappers.len(), 1);
        assert_eq!(project.no_delegates.len(), 1);
    }

    #[test]
    fn test_usage_rejects_bad_rule_exemption() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[[lele.rules.lint.no_collection_newtype.exempt]]\ntype = \"Deck\"\n",
        )
        .unwrap();
        let config = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let error = project.apply_layout(&config).unwrap_err();
        assert!(matches!(error, Error::BadRuleExempt { .. }), "{error:?}");
        assert!(error.to_string().contains("lint.no_collection_newtype"));
    }

    #[test]
    fn test_usage_accepts_boundary() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[[lele.project.boundary]]\nname = \"session\"\nwhy = \"pure\"\nfolders = [\"src/session\"]\ncannot_use = [\"tokio\"]\nrequire = \"honest\"\n",
        )
        .unwrap();
        let config = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap();
        let section = config.as_ref().unwrap();
        assert_eq!(section.project.boundary.len(), 1);
        assert_eq!(
            section.project.boundary.first().map(|b| b.name.as_str()),
            Some("session")
        );
        assert_eq!(
            section.project.boundary.first().and_then(|b| b.require),
            Some(Requirement::Honest)
        );
    }

    #[test]
    fn test_usage_rejects_unknown_boundary_key() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[[lele.project.boundary]]\nname = \"session\"\nwhy = \"pure\"\nfolders = [\"src/session\"]\nunknown = 1\n",
        )
        .unwrap();
        let error = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap_err();
        let err = std::error::Error::source(&error)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(err.contains("unknown"), "{err}");
    }

    #[test]
    fn test_usage_ignores_other_tools_sections() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[honesty]\nhonesty_depth = 1\n\n[ui_preview.bevy]\nresource = \"Snapshot\"\n\n[lele.project]\nexclude = [\"target\"]\n",
        )
        .unwrap();
        assert!(Config::try_from(dir.path().join("lele.toml").as_path())
            .unwrap()
            .as_ref()
            .is_some());
    }
}
