use std::path::Path;
use std::path::PathBuf;

use crate::Config;
use crate::Error;

pub fn load_at(path: &Path, fallback: &Path) -> Result<(Config, PathBuf), Error> {
    let path = path.canonicalize().map_err(Error::Io)?;
    let dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| fallback.to_path_buf());
    Ok((Config::try_from(path.as_path())?, dir))
}

// no test_usage necessary
