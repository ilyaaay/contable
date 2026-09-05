use bollard::{
    self, Docker, errors::Error, plugin::ImageSummary, query_parameters::ListImagesOptions,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DockerData {
    pub images: Images,
}

#[derive(Debug, Serialize)]
pub struct Images(pub Vec<ImageSummary>);

impl Images {
    pub fn get_strings(&self) -> Vec<String> {
        let mut list = Vec::new();

        for x in &self.0 {
            if let Ok(s) = serde_json::to_string(&x) {
                list.push(s);
            }
        }

        list
    }
}

pub struct DockerDaemon(Docker);

impl DockerDaemon {
    pub async fn connect() -> Result<Self, Error> {
        let connection = Docker::connect_with_defaults()?;

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
