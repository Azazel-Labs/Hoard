//! `hoardd`: the Hoard asset server.

use anyhow::Result;
use clap::Parser as _;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("hoardd=info,tower_http=warn"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .with(hoardd::recent_log())
        .init();
    hoardd::run(hoardd::Config::parse()).await
}
