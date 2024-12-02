use std::time::UNIX_EPOCH;

use anyhow::Result;
use axum::{
    extract::{Multipart, Path},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
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

pub async fn docker_packages_handler() -> Result<impl IntoResponse, AppError> {
    let base_folder = get_settings().base_folder.clone();
    let docker_pkgs_folder = format!("{}/docker_pkgs", base_folder);
    let mut entries = fs::read_dir(docker_pkgs_folder).await?;

    #[derive(Serialize)]
    struct PackageInfo {
        index: u16,
        name: String,
        create_time: Option<u64>,
    }

    let mut pkgs = Vec::new();
    let mut index = 0u16;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let metadata = entry.metadata().await?;
        let create_time = metadata.created().ok().and_then(|time| {
            time.duration_since(UNIX_EPOCH)
                .ok()
                .map(|duration| duration.as_secs())
        });
        index += 1;
        pkgs.push(PackageInfo {
            index,
            name,
            create_time,
        });
    }
    Ok(Json(pkgs))
}

pub async fn docker_install_handler(
    Path(package_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let base_folder = get_settings().base_folder.clone();
    let docker_pkgs_folder = format!("{}/docker_pkgs", base_folder);
    let file_path = format!("{}/{}", docker_pkgs_folder, package_name);

    // check if the file exists
    if !fs::metadata(&file_path).await?.is_file() {
        return Err(AppError::NotFound("docker package file not found".to_string()));
    }

    info!("installing docker package: {}", package_name);

    Ok(())
}
