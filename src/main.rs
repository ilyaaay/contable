mod docker;
mod error;
mod ui;

use crate::{
    docker::DockerDaemon,
    error::Error::{self},
    ui::app,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let docker_daemon = DockerDaemon::connect().await?;
    let docker_data = docker_daemon.collect_data().await?;

    ratatui::run(|x| app(x, &docker_data))?;

    Ok(())
}
