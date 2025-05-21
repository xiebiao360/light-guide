pub mod error;
pub mod handlers;

use std::{ops::Deref, sync::Arc};

use anyhow::Context;
use axum::{routing::get, Router};
use error::AppError;
use handlers::index_handler;
use sqlx::SqlitePool;

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
        let db = SqlitePool::connect(db_path)
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

pub async fn load_router(state: AppState) -> Result<axum::Router, AppError> {
    let router = Router::new()
        .route("/api/v1/health", get(index_handler))
        .with_state(state);
    Ok(router)
}
