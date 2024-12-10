use core::fmt;
use std::{fs::File, io::Read, ops, path::Path, sync::Arc};

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use daemonize::Daemonize;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tokio::{fs, net::TcpListener};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use crate::{
    docker_version_handler, error::AppError, index_handler, not_found, static_handler, RunArgs,
};

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

        Ok(Self {
            inner: Arc::new(AppStateInner { pool }),
        })
    }
}

const PID_FILE: &str = "/tmp/guide-web.pid";

pub fn run_server(args: &RunArgs) -> Result<()> {
    if args.detach {
        // 配置守护进程
        let stdout = File::create("/tmp/guide-web.out").unwrap();
        let stderr = File::create("/tmp/guide-web.err").unwrap();
        let daemonize = Daemonize::new()
            .pid_file(PID_FILE) // PID 文件
            .chown_pid_file(true) // 是否修改 PID 文件的所有者
            .working_directory("/tmp")
            .stdout(stdout) // 标准输出重定向
            .stderr(stderr); // 标准错误重定向

        match daemonize.start() {
            Ok(_) => {
                info!("Start light-guide-web successfully!");
                start_server(&args)
            }
            Err(e) => return Err(e.into()),
        }
    } else {
        start_server(&args)
    }
}

#[tokio::main]
pub async fn start_server(args: &RunArgs) -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let state = AppState::try_new(&args.db).await?;

    let docker = Router::new().route("/version", get(docker_version_handler));
    let api = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/docker", docker);

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

pub fn stop_server() -> Result<()> {
    if let Ok(mut file) = File::open(PID_FILE) {
        let mut pid = String::new();
        if file.read_to_string(&mut pid).is_ok() {
            if let Ok(pid) = pid.trim().parse::<i32>() {
                // stop pid process use Command
                let output = std::process::Command::new("kill")
                    .arg("-TERM")
                    .arg(format!("{}", pid))
                    .output()?;
                if output.status.success() {
                    info!("Stop light-guide-web successfully!");
                } else {
                    return Err(anyhow::anyhow!(
                        "Failed to stop light-guide-web. Error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
        }
    }
    Ok(())
}
