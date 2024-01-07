use thiserror::Error;

// Custom Error type
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Generic {0}")]
    Generic(String),
    #[error("{0}")]
    Static(&'static str),
}

// Custom Result type
pub type Result<T> = std::result::Result<T, Error>;

/// Shorhand for format!
pub use std::format as f;

/// Shorthand for OK(())
pub const OK: Result<()> = Ok(());
