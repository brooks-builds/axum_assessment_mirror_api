mod mirror;

use crate::config::Config;
use axum::{routing::post, Router};
use eyre::Result;
use mirror::mirror;

pub async fn create_router(config: &Config) -> Result<Router> {
    let router = Router::new().route("/:id", post(mirror));

    Ok(router)
}
