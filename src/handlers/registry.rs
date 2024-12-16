use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    error::AppError,
    models::registry::{CreateRegistryInput, SslType},
    utils::generate_self_signed_file,
    web_server::AppState,
};

pub async fn create_registry_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateRegistryInput>,
) -> Result<impl IntoResponse, AppError> {
    // 判断名称、主机、端口是否为空
    if input.name.is_empty() || input.host.is_empty() || input.port == 0 {
        return Err(AppError::InputParamsError(
            "name, host, port can't be empty".to_string(),
        ));
    }

    if let Some(registry) = state.get_registry_by_name(&input.name).await? {
        return Err(AppError::InputParamsError(format!(
            "registry {} already exists",
            registry.name
        )));
    }

    if let Some(ssl) = input.ssl {
        let (cert_file, key_file) = match ssl {
            SslType::SelfSigned(ss) => {
                generate_self_signed_file(&ss.cert_dir).await?;
                (
                    format!("{}/registry.crt", &ss.cert_dir),
                    format!("{}/registry.key", &ss.cert_dir),
                )
            }
            SslType::Provided(p) => (p.cert_file, p.key_file),
        };
    }

    Ok(())
}
