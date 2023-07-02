use std::net::SocketAddr;

use config::Config;
use eyre::Result;
use router::create_router;

pub mod config;
mod router;

pub async fn run(config: Config) -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = create_router(&config).await?;
    let address = SocketAddr::from((config.address, config.port));

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    tracing::debug!("server running on port {}", config.port);

    Ok(())
}
