use std::fs;
use std::path::Path;

pub(crate) fn write_source_file(base: &Path, relative_path: &str) {
    let path = base.join(relative_path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, "pub fn f() {}\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::write_source_file;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        write_source_file(dir.path(), "src/a.rs");
        assert!(dir.path().join("src/a.rs").exists());
    }
}
