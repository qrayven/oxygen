use serde::ser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("Unsupported: {0}")]
    Unsupported(String),
}

impl Error {
    pub fn unsupported(msg: &str) -> Self {
        Self::Unsupported(String::from(msg))
    }

    pub fn serialization(msg: &str) -> Self {
        Self::SerializationError(String::from(msg))
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::SerializationError(format!("{:#}", msg))
    }
}
