use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO-Error: {0}")]
    IOError(#[from] std::io::Error)
}

pub type Result<T> = std::result::Result<T, Error>;