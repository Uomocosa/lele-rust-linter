use crate::Project;

pub fn crate_edition(project: &Project) -> String {
    project
        .root
        .ancestors()
        .filter_map(|dir| std::fs::read_to_string(dir.join("Cargo.toml")).ok())
        .filter_map(|text| toml::from_str::<toml::Value>(&text).ok())
        .find_map(|value| {
            value
                .get("package")
                .and_then(|package| package.get("edition"))
                .and_then(toml::Value::as_str)
                .or_else(|| {
                    value
                        .get("workspace")?
                        .get("package")?
                        .get("edition")?
                        .as_str()
                })
                .map(str::to_string)
        })
        .unwrap_or_else(|| "2021".to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Project;

    #[test]
    fn test_usage() {
        let missing = Project {
            root: PathBuf::from("/nonexistent-lele-root"),
            ..Project::default()
        };
        assert_eq!(missing.crate_edition(), "2021");
        let here = Project {
            root: PathBuf::from("."),
            ..Project::default()
        };
        assert_eq!(here.crate_edition(), "2021");
    }

    #[test]
    fn test_usage_workspace_inherited_edition() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"member\"]\n[workspace.package]\nedition = \"2024\"\n",
        )
        .unwrap();
        let member = dir.path().join("member");
        std::fs::create_dir_all(&member).unwrap();
        std::fs::write(
            member.join("Cargo.toml"),
            "[package]\nname = \"member\"\nedition.workspace = true\n",
        )
        .unwrap();
        let project = Project {
            root: member,
            ..Project::default()
        };
        assert_eq!(project.crate_edition(), "2024");
    }
}
