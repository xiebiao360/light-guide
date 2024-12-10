use anyhow::{Ok, Result};
// use light_guide::DockerClient;

#[tokio::main]
async fn main() -> Result<()> {
    // let docker = DockerClient::try_new()?;
    // let image = docker.get_image_by_name("redis:6").await?;
    // println!("{:?}", image);

    let registry = light_guide::registry::Registry::try_new()?;
    // registry.is_container_exists("trusting_shannon").await?;
    let b = registry.is_image_exists("dis:6").await?;
    println!("{}", b);
    Ok(())
}
