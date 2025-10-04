use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Network(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Clipboard(#[from] arboard::Error),
    #[error(transparent)]
    Dialogue(#[from] dialoguer::Error),
    #[error("{0}")]
    ResultNotFound(String),
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
