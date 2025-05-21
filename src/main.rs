use anyhow::Result;
use light_guide::AppState;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let db_path = "sqlite://test.db";
    let state = AppState::try_new(db_path).await?;
    info!("Database connection established");

    Ok(())
}
