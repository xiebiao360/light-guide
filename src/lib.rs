pub mod error;
pub mod handlers;

use std::{ops::Deref, sync::Arc};

use anyhow::{Context, anyhow};
use axum::{
    Router,
    extract::Path,
    http::header,
    response::{IntoResponse, Redirect},
    routing::get,
};
use error::AppError;
use handlers::health_handler;
use sqlx::SqlitePool;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct AppStateInner {
    pub(crate) db: SqlitePool,
}

impl AppState {
    pub async fn try_new(db_path: &str) -> Result<Self, AppError> {
        // check if the database file exists. e.g. "sqlite://guide.db"
        let db_path = if db_path.starts_with("sqlite://") {
            db_path[9..].to_string()
        } else {
            db_path.to_string()
        };
        if !db_path.ends_with(".db") {
            return Err(anyhow!("Invalid database path: {}", db_path).into());
        }
        // if the database file not exists, create it
        if !std::path::Path::new(&db_path).exists() {
            std::fs::File::create(&db_path)
                .with_context(|| format!("Failed to create database file: {}", db_path))?;
        }

        let db = SqlitePool::connect(&db_path)
            .await
            .context("Failed to connect to database")?;
        Ok(Self {
            inner: Arc::new(AppStateInner { db }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(rust_embed::RustEmbed)]
#[folder = "static/"]
struct Asset;

pub async fn load_router(state: AppState) -> Result<axum::Router, AppError> {
    let router = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/static/index.html") }),
        )
        .route("/static/{*path}", get(serve_static_files))
        .route("/api/v1/health", get(health_handler))
        .with_state(state);
    Ok(router)
}

async fn serve_static_files(Path(path): Path<String>) -> impl IntoResponse {
    info!("Serving static file: {}", path);

    if let Some(file) = Asset::get(&path) {
        let content_type = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .to_string();
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, "max-age=31536000".to_string()), // 缓存一年
        ];
        return (axum::http::StatusCode::OK, headers, file.data.into_owned());
    }
    let headers = [
        (header::CONTENT_TYPE, "text/plain".to_string()),
        (header::CACHE_CONTROL, "no-cache".to_string()),
    ];
    let body = format!("File not found: {}", path);
    let status = axum::http::StatusCode::NOT_FOUND;
    (status, headers, body.as_bytes().to_vec())
}
