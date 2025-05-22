use axum::response::IntoResponse;

pub(crate) async fn health_handler() -> impl IntoResponse {
    "healthy"
}
