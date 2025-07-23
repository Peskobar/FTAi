use anyhow::Result;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    info!("Uruchomiono tryb edge");
    tokio::signal::ctrl_c().await?;
    info!("Zamykanie trybu edge");
    Ok(())
}
