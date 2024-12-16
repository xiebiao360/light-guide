mod docker;
mod registry;
mod sse;
mod static_file;

use axum::{
    http::Uri,
    response::{Html, IntoResponse},
};

pub use docker::*;
pub use registry::*;
pub use sse::sse_handler;
pub use static_file::static_handler;

pub async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

pub async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}
