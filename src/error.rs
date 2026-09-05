use bollard::errors;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    DockerDaemon(errors::Error),
    Io(io::Error),
    Serde(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DockerDaemon(error) => write!(f, "docker daemon error: {:?}", error),
            Error::Io(error) => write!(f, "io error: {:?}", error),
            Error::Serde(error) => write!(f, "serde error: {:?}", error),
        }
    }
}

impl From<errors::Error> for Error {
    fn from(value: errors::Error) -> Self {
        Self::DockerDaemon(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
