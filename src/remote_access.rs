use ssh::Session;
use std::{env, io};

#[derive(Debug)]
pub enum ConnectionError {
    Ssh(ssh::Error),
    Io(io::Error),
}

impl From<ssh::Error> for ConnectionError {
    fn from(value: ssh::Error) -> Self {
        Self::Ssh(value)
    }
}

fn connect() -> Result<(), ConnectionError> {
    let mut session =
        Session::new().map_err(|_| ssh::Error::Ssh("Create ssh session error".into()))?;

    session.parse_config(None)?;

    Ok(())
}
