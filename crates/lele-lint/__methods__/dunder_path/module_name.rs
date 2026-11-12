use crate::DunderPath;

pub fn module_name(dunder_path: &DunderPath) -> &str {
    dunder_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.strip_prefix("__"))
        .and_then(|rest| rest.strip_suffix("__"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::module_name;
    use crate::DunderPath;

    #[test]
    fn test_usage() {
        let dir = DunderPath::try_from(PathBuf::from("__basic__")).unwrap();
        assert_eq!(module_name(&dir), "basic");
        let file = DunderPath::try_from(PathBuf::from("src/__notes__.md")).unwrap();
        assert_eq!(module_name(&file), "notes");
    }
}
