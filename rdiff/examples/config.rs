use anyhow::Result;
use rdiff::{DiffConfig, LoadConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = DiffConfig::load_yaml_config("yaml/request.yml").await?;
    println!("{:#?}", config);
    Ok(())
}
