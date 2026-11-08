use std::path::PathBuf;

use crate::Config;

pub fn layout_roots(config: &Config) -> Vec<PathBuf> {
    let Some(section) = config.as_ref() else {
        return vec![PathBuf::from("src")];
    };
    if section.layout.roots.is_empty() {
        return vec![PathBuf::from("src")];
    }
    section.layout.roots.clone()
}

#[cfg(test)]
mod tests {
    use super::layout_roots;
    use crate::Config;
    use crate::LeleSection;

    #[test]
    fn test_usage() {
        assert_eq!(
            layout_roots(&Config::default()),
            vec![std::path::PathBuf::from("src")]
        );
        let section: LeleSection = toml::from_str("[layout]\nroots = [\"src\", \"ui\"]\n").unwrap();
        assert_eq!(
            layout_roots(&Config(Some(section))),
            vec![
                std::path::PathBuf::from("src"),
                std::path::PathBuf::from("ui")
            ]
        );
    }
}
