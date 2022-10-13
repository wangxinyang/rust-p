use anyhow::Result;
use tonicr::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    server::start().await;
    Ok(())
}
