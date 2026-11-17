//! `hoardd`: the Hoard asset server.

use anyhow::Result;
use clap::Parser as _;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("hoardd=info,tower_http=warn")),
        )
        .init();
    hoardd::run(hoardd::Config::parse()).await
}
