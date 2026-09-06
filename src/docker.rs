use bollard::{
    self, Docker,
    errors::Error,
    plugin::{ContainerSummary, ImageSummary},
    query_parameters::{ListContainersOptions, ListImagesOptions},
};
use serde::Serialize;
use time::{OffsetDateTime, macros::format_description};

#[derive(Debug, Serialize)]
pub struct DockerData {
    pub images: Images,
    pub containers: Containers,
}

#[derive(Debug, Serialize)]
pub struct Images(pub Vec<ImageSummary>);

#[derive(Debug, Serialize)]
pub struct Containers(pub Vec<ContainerSummary>);

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

impl Containers {
    pub fn get_containers(&self) -> Vec<String> {
        let datetime_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let mut list = Vec::new();

        for container in &self.0 {
            let id = container.id.as_deref().unwrap_or("ID not found");
            let names = container.names.as_deref().unwrap_or_default().join(",");

            let image = container.image.as_deref().unwrap_or_default();
            let status = container.status.as_deref().unwrap_or_default();
            let state = container.state.map(|s| s.to_string()).unwrap_or_default();

            let ports: Vec<String> = container
                .ports
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|port| {
                    let proto = port.typ.as_ref().map(|t| t.to_string()).unwrap_or_default();
                    match port.public_port {
                        Some(public) if !proto.is_empty() => {
                            format!("{}/{}->{}", port.private_port, proto, public)
                        }
                        Some(public) => format!("{}->{}", port.private_port, public),
                        None if !proto.is_empty() => {
                            format!("{}/{}", port.private_port, proto)
                        }
                        None => port.private_port.to_string(),
                    }
                })
                .collect();

            let created = container
                .created
                .and_then(|x| OffsetDateTime::from_unix_timestamp(x).ok())
                .and_then(|x| x.format(&datetime_format).ok())
                .unwrap_or_else(|| container.created.map(|x| x.to_string()).unwrap_or_default());

            list.push(format!("ID:      {}", id));
            list.push(format!("Names:   {}", names));
            list.push(format!("Image:   {}", image));
            list.push(format!("State:   {}", state));
            list.push(format!("Status:  {}", status));
            list.push(format!("Ports:   {}", ports.join(",")));
            list.push(format!("Created: {}", created));
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

    async fn get_containers(&self) -> Result<Vec<ContainerSummary>, Error> {
        let options = ListContainersOptions {
            all: true,
            limit: None,
            size: false,
            filters: None,
        };

        let x = self.0.list_containers(Some(options)).await?;

        Ok(x)
    }

    pub async fn collect_data(&self) -> Result<DockerData, Error> {
        let images = self.get_images().await?;
        let containers = self.get_containers().await?;

        let docker_data = DockerData {
            images: Images(images),
            containers: Containers(containers),
        };

        Ok(docker_data)
    }
}
