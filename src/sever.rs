use core::fmt;
use std::{ops, path::Path, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::{fs, net::TcpListener, sync::broadcast};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use crate::{
    docker_install_handler, docker_packages_handler, docker_remove_handler, docker_upload_handler,
    docker_version_handler, error::AppError, index_handler, not_found, sse_handler, static_handler,
    AppEvent, AppSettings, RunArgs,
};

pub type AgentMap = Arc<DashMap<String, broadcast::Sender<Arc<AppEvent>>>>;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub struct AppStateInner {
    pub(crate) pool: SqlitePool,
    pub(crate) agents: AgentMap,
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
    pub async fn try_new(db: &str) -> Result<Self, AppError> {
        let db_url = format!("sqlite://{}", db);

        if let Some(parent) = Path::new(db).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .await
                    .context("Failed to create database directory")?;
            }
        }

        if !Sqlite::database_exists(&db_url).await? {
            info!("Creating database at {}", db_url);
            Sqlite::create_database(&db_url).await?;
            info!("Database created");
        }
        let pool = SqlitePool::connect(&db_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let agents = Arc::new(DashMap::new());

        Ok(Self {
            inner: Arc::new(AppStateInner { pool, agents }),
        })
    }
}

pub async fn start_server(args: &RunArgs) -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let state = AppState::try_new(&args.db).await?;

    AppSettings::try_load_from_db(&state).await?;

    let docker = Router::new()
        .route("/version", get(docker_version_handler))
        .route(
            "/upload",
            post(docker_upload_handler).layer(DefaultBodyLimit::max(1024 * 1024 * 100)),
        )
        .route("/packages", get(docker_packages_handler))
        .route("/install/:package_name", post(docker_install_handler))
        .route("/remove/:package_name", post(docker_remove_handler));
    let api = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/docker", docker);

    let app = Router::new()
        .route("/events", get(sse_handler))
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
