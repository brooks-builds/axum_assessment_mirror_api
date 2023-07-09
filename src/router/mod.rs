mod mirror;

use axum::{routing::post, Router};
use eyre::Result;
use mirror::mirror;
use tower_http::trace::DefaultOnRequest;
use tracing::Level;

pub async fn create_router() -> Result<Router> {
    let router = Router::new().route("/:id", post(mirror)).layer(
        tower_http::trace::TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::TRACE)),
    );

    Ok(router)
}
