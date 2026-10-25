use std::path::Path;

use derive_more::Deref;

use crate::Error;

#[derive(Deref)]
pub struct ConfigText(pub String);

impl TryFrom<&Path> for ConfigText {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path).map_err(|source| Error::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Ok(Self(text))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ConfigText;

    #[test]
    fn test_usage() {
        assert!(ConfigText::try_from(Path::new("/nonexistent")).is_err());
    }
}
