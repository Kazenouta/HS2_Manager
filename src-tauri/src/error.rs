#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

