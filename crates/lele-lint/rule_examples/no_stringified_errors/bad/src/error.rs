use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {path}: {message}")]
    Read { path: PathBuf, message: String },
}
