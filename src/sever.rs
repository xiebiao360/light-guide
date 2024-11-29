use core::fmt;
use std::{fs, ops, path::Path, sync::Arc};

use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use crate::{index_handler, not_found, static_handler, RunArgs};

#[derive(Debug, Clone)]
struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
struct AppStateInner {
    pub(crate) pool: SqlitePool,
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner").finish()
    }
}

impl ops::Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(db: &str) -> Result<Self> {
        let db_url = format!("sqlite://{}", db);

        if let Some(parent) = Path::new(db).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if !Sqlite::database_exists(&db_url).await? {
            info!("Creating database at {}", db_url);
            Sqlite::create_database(&db_url).await?;
            info!("Database created");
        }
        let pool = SqlitePool::connect(&db_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self {
            inner: Arc::new(AppStateInner { pool }),
        })
    }
}

pub async fn start_server(args: &RunArgs) -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let state = AppState::try_new(&args.db).await?;

    let api = Router::new().route("/", get(|| async { "Hello, World!" }));

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/dist/*file", get(static_handler))
        .nest("/api", api)
        .fallback_service(get(not_found))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

pub async fn stop_server() -> Result<()> {
    Ok(())
}
