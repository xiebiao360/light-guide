use std::{collections::HashMap, ops, process::Command};

use anyhow::{anyhow, Result};
use bollard::{
    container::ListContainersOptions,
    errors::Error,
    image::{CreateImageOptions, ImportImageOptions, ListImagesOptions},
    Docker,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct DockerClient(Docker);

#[derive(Debug)]
pub struct ContainerInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug)]
pub struct ImageInfo {
    pub id: String,
    pub repo_tags: Option<Vec<String>>,
}

impl DockerClient {
    pub fn try_new() -> Result<Self> {
        match Docker::connect_with_local_defaults() {
            Ok(docker) => Ok(DockerClient(docker)),
            Err(e) => match e {
                Error::DockerResponseServerError {
                    status_code,
                    message,
                } => {
                    if status_code == 500 {
                        // 尝试启动 Docker 服务
                        let output = Command::new("sudo")
                            .arg("systemctl")
                            .arg("start")
                            .arg("docker")
                            .output()?;

                        if output.status.success() {
                            Ok(DockerClient(Docker::connect_with_local_defaults()?))
                        } else {
                            Err(anyhow!(
                                "Failed to start Docker service. Error: {}",
                                String::from_utf8_lossy(&output.stderr)
                            ))
                        }
                    } else {
                        Err(anyhow!("Failed to connect to Docker: {}", message))
                    }
                }
                _ => Err(anyhow!("Failed to connect to Docker: {}", e.to_string())),
            },
        }
    }
    pub async fn get_image_by_name(&self, image_name: &str) -> Result<Option<ImageInfo>> {
        let mut filters = HashMap::new();
        filters.insert("reference", vec![image_name]);
        let options = Some(ListImagesOptions {
            all: true,
            filters,
            ..Default::default()
        });
        let images = self.list_images(options).await?;
        for image in images {
            for tag in &image.repo_tags {
                if tag == image_name {
                    return Ok(Some(ImageInfo {
                        id: image.id,
                        repo_tags: Some(image.repo_tags),
                    }));
                }
            }
        }
        Ok(None)
    }
    pub async fn load_image_file(&self, image_file: &str) -> Result<()> {
        let file = tokio::fs::File::open(image_file).await?;
        let mut byte_stream = FramedRead::new(file, BytesCodec::new()).map(|r| {
            let bytes = r.unwrap().freeze();
            Ok::<_, Error>(bytes)
        });
        let bytes = byte_stream.next().await.unwrap().unwrap();
        let mut stream = self.import_image(
            ImportImageOptions {
                ..Default::default()
            },
            bytes,
            None,
        );
        while let Some(item) = stream.try_next().await? {
            if let Some(progress) = item.progress_detail {
                // println!("{:?}", progress);
                // print percentage
                if let Some(total) = progress.total {
                    if let Some(current) = progress.current {
                        let percentage = (current as f64 / total as f64) * 100.0;
                        print!("\r{:.2}%", percentage);
                    }
                }
            }
        }
        Ok(())
    }
    pub async fn pull_image(&self, image_name: &str) -> Result<()> {
        let options = Some(CreateImageOptions {
            from_image: image_name,
            ..Default::default()
        });
        let mut stream = self.create_image(options, None, None);
        while let Some(item) = stream.try_next().await? {
            if let Some(progress) = item.progress_detail {
                // println!("{:?}", progress);
                if let Some(total) = progress.total {
                    if let Some(current) = progress.current {
                        let percentage = (current as f64 / total as f64) * 100.0;
                        print!("\r{:.2}%", percentage);
                    }
                }
            }
        }
        println!();
        println!("Image {} pulled successfully", image_name);
        Ok(())
    }
    pub async fn get_container_by_name(
        &self,
        container_name: &str,
    ) -> Result<Option<ContainerInfo>> {
        let mut filters = HashMap::new();
        filters.insert("name", vec![container_name]);
        let options = Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        });
        let containers = self.list_containers(options).await?;
        for container in containers {
            if let Some(names) = container.names {
                for name in names {
                    if name == format!("/{}", container_name) {
                        return Ok(Some(ContainerInfo {
                            id: container.id,
                            name: Some(container_name.to_string()),
                            image: container.image,
                            state: container.state,
                        }));
                    }
                }
            }
        }
        Ok(None)
    }
}

impl ops::Deref for DockerClient {
    type Target = Docker;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
