use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use axum_extra::TypedHeader;
use futures::Stream;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::{debug, info};

use crate::{web_server::AppState, AppEvent};

pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("User-Agent: {} connected", user_agent);

    let agents = &state.agents;
    let rx = if let Some(tx) = agents.get(user_agent.as_str()) {
        tx.subscribe()
    } else {
        let (tx, rx) = broadcast::channel(100);
        state.agents.insert(user_agent.to_string(), tx);
        rx
    };

    let stream = BroadcastStream::new(rx).filter_map(|v| v.ok()).map(|v| {
        let name = match v.as_ref() {
            AppEvent::InstallDocker(_) => "install_docker",
        };
        let v = serde_json::to_string(&v).expect("Failed to serialize event");
        debug!("Sending event: {}: {}", name, v);
        Ok(Event::default().data(v))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
