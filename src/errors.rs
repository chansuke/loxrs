use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Failed to handle IO error")]
    IOError(#[from] std::io::Error),
}
