use anyhow::{Ok, Result};
use bollard::{container::ListContainersOptions, Docker};

#[tokio::main]
async fn main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    // let version = docker.version().await?;

    // println!("{:?}", version);

    // let info = docker.info().await?;

    // println!("{:?}", info);

    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });
    let containers = docker.list_containers(options).await?;

    for container in containers {
        println!("{:?}", container);
    }

    Ok(())
}
