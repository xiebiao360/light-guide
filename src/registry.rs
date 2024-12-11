use std::{collections::HashMap, fs::File, io::Read, ops::Deref, path::Path};

use anyhow::Result;
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    models::PortBinding,
};
use bollard_stubs::models::HostConfig;

use crate::{
    utils::{generate_self_signed_file, get_local_ip},
    DockerClient, RegistryInitArgs,
};

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

    pub async fn run(&self, args: &RegistryInitArgs) -> Result<()> {
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
                host_port: Some(args.port.to_string()),
            }]),
        );

        let cert_dir = if Path::new(&args.cert_dir).is_absolute() {
            args.cert_dir.clone()
        } else {
            format!("{}/{}", std::env::current_dir()?.display(), args.cert_dir)
        };
        let host_config = HostConfig {
            binds: Some(vec![
                "/var/lib/registry:/var/lib/registry".to_string(),
                format!("{}:/certs", cert_dir),
            ]),
            port_bindings: Some(port_bindings),
            ..Default::default()
        };
        let config = Config {
            image: Some(args.image.as_str()),
            env: Some(vec![
                "REGISTRY_HTTP_ADDR=0.0.0.0:5000",
                "REGISTRY_HTTP_TLS_CERTIFICATE=/certs/registry.crt",
                "REGISTRY_HTTP_TLS_KEY=/certs/registry.key",
            ]),
            host_config: Some(host_config),
            ..Default::default()
        };
        println!("Creating registry container {}...", args.name);
        // 生成证书
        generate_self_signed_file(&args.cert_dir).await?;
        println!("Certificate generated successfully");
        // 创建容器
        let container = self.create_container(options, config).await?;
        self.start_container(&container.id, None::<StartContainerOptions<String>>)
            .await?;

        println!("Registry container {} started", args.name);
        println!("Registry is running on https://localhost:{}", args.port);

        let cert_file = Path::new(&args.cert_dir).join("registry.crt");
        // 读取证书文件内容并打印
        let mut file = File::open(cert_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        println!("Certificate content: \n{}", content);

        let local_ip = get_local_ip().unwrap_or_default();
        println!("==================镜像拉取配置=====================");
        println!(
            "请将以上证书内容复制到 /etc/docker/certs.d/{}:{}/ca.crt文件中",
            &local_ip, args.port
        );
        println!("或使用以下命令远程获取证书：");
        println!("echo | openssl s_client -showcerts -connect {}:{} 2>/dev/null | openssl x509 -outform PEM > /etc/docker/certs.d/{}:{}/ca.crt",
            &local_ip, args.port, &local_ip, args.port
        );

        Ok(())
    }
}

#[tokio::main]
pub async fn run_container(args: &RegistryInitArgs) -> Result<()> {
    let registry = Registry::try_new()?;
    registry.run(args).await?;
    Ok(())
}
