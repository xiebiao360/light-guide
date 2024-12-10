use std::{ops, process::Command};

use anyhow::{anyhow, Result};
use bollard::{
    errors::Error,
    image::{ImportImageOptions, ListImagesOptions},
    Docker,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct DockerClient(Docker);

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
    pub async fn get_images(&self) -> Result<()> {
        let options = Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        });
        let images = self.list_images(options).await?;
        for image in images {
            println!("{:?}", image.repo_tags);
        }
        Ok(())
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
                println!("{:?}", progress);
            }
        }
        Ok(())
    }
}

impl ops::Deref for DockerClient {
    type Target = Docker;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
