use bollard::{Docker, errors};
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    DockerDaemon(errors::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DockerDaemon(error) => write!(f, "docker daemon error: {:?}", error),
            Error::Io(error) => write!(f, "io error: {:?}", error),
        }
    }
}

impl From<errors::Error> for Error {
    fn from(value: errors::Error) -> Self {
        Self::DockerDaemon(value)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let connection = Docker::connect_with_defaults()?;

    connection.ping().await?;

    Ok(())
}
