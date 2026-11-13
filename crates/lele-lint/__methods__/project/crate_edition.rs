use crate::Project;

pub fn crate_edition(project: &Project) -> String {
    std::fs::read_to_string(project.root.join("Cargo.toml"))
        .ok()
        .and_then(|text| toml::from_str::<toml::Value>(&text).ok())
        .and_then(|value| {
            value
                .get("package")?
                .get("edition")?
                .as_str()
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
}
