use std::path::PathBuf;

use crate::Config;

pub fn layout_methods_root(config: &Config) -> PathBuf {
    let Some(section) = config.as_ref() else {
        return PathBuf::from("__methods__");
    };
    if section.layout.methods_root.as_os_str().is_empty() {
        return PathBuf::from("__methods__");
    }
    section.layout.methods_root.clone()
}

#[cfg(test)]
mod tests {
    use super::layout_methods_root;
    use crate::Config;
    use crate::LeleSection;

    #[test]
    fn test_usage() {
        assert_eq!(
            layout_methods_root(&Config::default()),
            std::path::PathBuf::from("__methods__")
        );
        let section: LeleSection =
            toml::from_str("[layout]\nmethods_root = \"custom_methods\"\n").unwrap();
        assert_eq!(
            layout_methods_root(&Config(Some(section))),
            std::path::PathBuf::from("custom_methods")
        );
    }
}
