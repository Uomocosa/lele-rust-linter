use std::path::Path;

use crate::Config;
use crate::Error;
use crate::LeleToml;

pub fn load_file(path: &Path) -> Result<Config, Error> {
    let content = std::fs::read_to_string(path).map_err(Error::Io)?;
    let parsed: LeleToml = toml::from_str(&content)?;
    Ok(Config(Some(parsed.lele)))
}

#[cfg(test)]
mod tests {
    use super::load_file;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.lint]\n").unwrap();
        assert!(load_file(&path).unwrap().as_ref().is_some());
    }
}
