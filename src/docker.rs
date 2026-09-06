use bollard::{
    self, Docker, errors::Error, plugin::ImageSummary, query_parameters::ListImagesOptions,
};
use serde::Serialize;
use time::{OffsetDateTime, macros::format_description};

#[derive(Debug, Serialize)]
pub struct DockerData {
    pub images: Images,
}

#[derive(Debug, Serialize)]
pub struct Images(pub Vec<ImageSummary>);

impl Images {
    pub fn get_images(&self) -> Vec<String> {
        let datetime_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let mut list = Vec::new();

        for image in &self.0 {
            list.push(format!("ID:         {}", image.id));
            list.push(format!("Tags:       {}", image.repo_tags.join(",")));
            list.push(format!("Size:       {} bytes", image.size));
            list.push(format!("SharedSize: {} bytes", image.shared_size));

            let created = OffsetDateTime::from_unix_timestamp(image.created)
                .ok()
                .and_then(|x| x.format(&datetime_format).ok())
                .unwrap_or_else(|| image.created.to_string());
            list.push(format!("Created:    {}", created));
            list.push(format!("Active containers: {}", image.containers));
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
