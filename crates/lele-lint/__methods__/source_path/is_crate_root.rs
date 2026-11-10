use crate::common;

pub fn is_crate_root(path: &common::SourcePath) -> bool {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(file_name, "lib.rs" | "main.rs")
}

#[cfg(test)]
mod tests {
    use super::is_crate_root;
    use crate::common;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(is_crate_root(&common::SourcePath(Path::new("main.rs"))));
        assert!(!is_crate_root(&common::SourcePath(Path::new(
            "player/mod.rs"
        ))));
    }
}
