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

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.lint]\n").unwrap();
        assert!(Config::try_from(path.as_path()).unwrap().as_ref().is_some());
    }

    #[test]
    fn test_usage_rejects_unknown_lele_keys() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[lele.lint.checkers]\ncontainer_placement = false\n",
        )
        .unwrap();
        let err = Config::try_from(dir.path().join("lele.toml").as_path())
            .unwrap_err()
            .to_string();
        assert!(err.contains("checkers"), "{err}");
    }

    #[test]
    fn test_usage_accepts_boundary() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[[lele.boundary]]\nname = \"session\"\nwhy = \"pure\"\nfolders = [\"src/session\"]\ncannot_use = [\"tokio\"]\nrequire = \"honest\"\n",
        )
        .unwrap();
        let config = Config::try_from(dir.path().join("lele.toml").as_path()).unwrap();
        let section = config.as_ref().unwrap();
        assert_eq!(section.boundary.len(), 1);
        assert_eq!(
            section.boundary.first().map(|b| b.name.as_str()),
            Some("session")
        );
        assert_eq!(
            section.boundary.first().and_then(|b| b.require),
            Some(crate::Requirement::Honest)
        );
    }

    #[test]
    fn test_usage_rejects_unknown_boundary_key() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[[lele.boundary]]\nname = \"session\"\nwhy = \"pure\"\nfolders = [\"src/session\"]\nunknown = 1\n",
        )
        .unwrap();
        let err = Config::try_from(dir.path().join("lele.toml").as_path())
            .unwrap_err()
            .to_string();
        assert!(err.contains("unknown"), "{err}");
    }

    #[test]
    fn test_usage_ignores_other_tools_sections() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("lele.toml"),
            "[honesty]\nhonesty_depth = 1\n\n[ui_preview.bevy]\nresource = \"Snapshot\"\n\n[lele.config]\nexclude = [\"target\"]\n\n[lele.lint]\n",
        )
        .unwrap();
        assert!(Config::try_from(dir.path().join("lele.toml").as_path())
            .unwrap()
            .as_ref()
            .is_some());
    }
}
