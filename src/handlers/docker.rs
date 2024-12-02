use anyhow::Result;
use axum::{extract::Multipart, response::IntoResponse, Json};
use tokio::{fs, io::AsyncWriteExt};
use tracing::info;

use crate::{error::AppError, get_settings, models::DockerClient};

pub async fn docker_version_handler() -> Result<impl IntoResponse, AppError> {
    let version = DockerClient::try_new()?.version().await?;

    #[derive(serde::Serialize)]
    struct Version {
        version: Option<String>,
        api_version: Option<String>,
    }
    let resp = Version {
        version: version.version,
        api_version: version.api_version,
    };

    Ok(Json(resp))
}

pub async fn docker_upload_handler(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    while let Some(mut field) = multipart.next_field().await? {
        let name = field.name().unwrap();
        info!("uploading field: {}", name);
        let filename = field.file_name().unwrap();
        info!("filename: {}", filename);

        let base_folder = get_settings().base_folder.clone();
        let docker_pkgs_folder = format!("{}/docker_pkgs", base_folder);
        fs::create_dir_all(&docker_pkgs_folder).await?;

        let file_path = format!("{}/{}", docker_pkgs_folder, filename);

        let mut file = fs::File::create(file_path).await?;
        while let Some(chunk) = field.chunk().await? {
            info!("writing chunk of size: {}", chunk.len());
            file.write_all(&chunk).await?;
        }
    }
    Ok(())
}
