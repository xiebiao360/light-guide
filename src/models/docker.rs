use std::{ops, process::Command};

use anyhow::{anyhow, Result};
use bollard::{errors::Error, Docker};

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
}

impl ops::Deref for DockerClient {
    type Target = Docker;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}