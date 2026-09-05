use bollard::{
    self, Docker, errors::Error, plugin::ImageSummary, query_parameters::ListImagesOptions,
};
use serde::Serialize;
use std::{fmt, time::Duration};

#[derive(Debug, Serialize)]
pub struct DockerData {
    pub images: Images,
}

#[derive(Debug, Serialize)]
pub struct Images(pub Vec<ImageSummary>);

pub struct DockerDaemon(Docker);

impl DockerDaemon {
    pub async fn connect() -> Result<Self, Error> {
        let connection = Docker::connect_with_defaults()?.with_timeout(Duration::from_millis(2000));
        connection.ping().await?;

        Ok(Self(connection))
    }

    async fn get_images(&self) -> Result<Vec<ImageSummary>, Error> {
        let options = ListImagesOptions {
            all: true,
            filters: None,
            shared_size: true,
            digests: true,
            manifests: true,
        };

        let x = self.0.list_images(Some(options)).await?;

        Ok(x)
    }

    pub async fn collect_data(&self) -> Result<DockerData, Error> {
        let images = self.get_images().await?;

        let docker_data = DockerData {
            images: Images(images),
        };

        Ok(docker_data)
    }
}
