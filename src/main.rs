use anyhow::Result;
use light_guide::{AppState, load_router};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let db_path = "sqlite://guide.db";
    let state = AppState::try_new(db_path).await?;
    let app = load_router(state).await?;
    let addr: &str = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;

    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
