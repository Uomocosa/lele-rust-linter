#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error")]
    Io(#[from] std::io::Error),
}
