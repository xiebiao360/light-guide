use std::{collections::HashMap, ops::Deref};

use anyhow::Result;
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    models::PortBinding,
};
use bollard_stubs::models::HostConfig;

use crate::{DockerClient, RegistryArgs};

pub struct Registry(DockerClient);

impl Deref for Registry {
    type Target = DockerClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Registry {
    pub fn try_new() -> Result<Self> {
        let docker = DockerClient::try_new()?;
        Ok(Registry(docker))
    }

    pub async fn is_container_exists(&self, container_name: &str) -> Result<bool> {
        if let Some(container) = self.get_container_by_name(container_name).await? {
            return Ok(container.state.is_some());
        }
        Ok(false)
    }

    pub async fn is_image_exists(&self, image_name: &str) -> Result<bool> {
        let image = self.get_image_by_name(image_name).await?;
        Ok(image.is_some())
    }

    pub async fn run(&self, args: &RegistryArgs) -> Result<()> {
        if self.is_container_exists(&args.name).await? {
            println!("Container {} already exists", args.name);
            return Ok(());
        }
        if !self.is_image_exists(&args.image).await? {
            if let Some(image_file) = &args.image_file {
                self.load_image_file(image_file).await?;
            } else {
                self.pull_image(&args.image).await?;
            }
        }
        // start a registry container
        let options = Some(CreateContainerOptions {
            name: args.name.as_str(),
            platform: None,
        });

        // 配置端口映射
        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            "5000/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()), // 绑定到所有接口
                host_port: Some("5000".to_string()),
            }]),
        );

        let host_config = HostConfig {
            binds: Some(vec!["/var/lib/registry:/var/lib/registry".to_string()]),
            port_bindings: Some(port_bindings),
            ..Default::default()
        };

        let config = Config {
            image: Some(args.image.as_str()),
            host_config: Some(host_config),
            ..Default::default()
        };
        let container = self.create_container(options, config).await?;
        self.start_container(&container.id, None::<StartContainerOptions<String>>)
            .await?;

        println!("Registry container {} started", args.name);
        Ok(())
    }
}

#[tokio::main]
pub async fn run_container(args: &RegistryArgs) -> Result<()> {
    let registry = Registry::try_new()?;
    registry.run(args).await?;
    Ok(())
}
