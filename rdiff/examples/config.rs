use anyhow::Result;
use rdiff::DiffConfig;

fn main() -> Result<()> {
    let confi_str = include_str!("../yaml/request.yml");
    let config: DiffConfig = serde_yaml::from_str(confi_str)?;
    println!("{:#?}", config);
    Ok(())
}
