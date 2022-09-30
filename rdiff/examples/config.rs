use anyhow::Result;
use rdiff::Config;

fn main() -> Result<()> {
    let config = Config::new("yaml/request.yml")?;
    println!("{:#?}", config);
    Ok(())
}
