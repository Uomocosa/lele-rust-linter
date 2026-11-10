use crate::common;

pub fn is_index_file(path: &common::SourcePath) -> bool {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    matches!(file_name, "mod.rs" | "lib.rs" | "main.rs")
}

#[cfg(test)]
mod tests {
    use super::is_index_file;
    use crate::common;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(is_index_file(&common::SourcePath(Path::new("mod.rs"))));
        assert!(!is_index_file(&common::SourcePath(Path::new("greet.rs"))));
    }
}
