use anyhow::{Ok, Result};
use light_guide::DockerClient;

#[tokio::main]
async fn main() -> Result<()> {
    let docker = DockerClient::try_new()?;
    docker.get_images().await?;
    Ok(())
}
