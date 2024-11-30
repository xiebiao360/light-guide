use anyhow::Result;
use axum::{response::IntoResponse, Json};

use crate::{error::AppError, models::DockerClient};

pub async fn docker_version_handler() -> Result<impl IntoResponse, AppError> {
    let version = DockerClient::try_new()?.version().await?;

    #[derive(serde::Serialize)]
    struct Version {
        version: Option<String>,
    }
    let resp = Version {
        version: version.version,
    };

    Ok(Json(resp))
}
