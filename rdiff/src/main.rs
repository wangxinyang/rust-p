use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use rdiff::{Commands, DiffCli, DiffConfig, ExtraConfigs, RunArgs};
use std::io::Write;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // get config
    let config = DiffCli::parse();
    match config.command {
        Commands::Add(args) => run(args).await?,
        _ => panic!("Invalid command"),
    };
    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_str = fs::read_to_string(args.config).await?;
    let config: DiffConfig = serde_yaml::from_str(&config_str)?;

    let profile = config.get_profiles(&args.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_str
        )
    })?;

    let extra_args: ExtraConfigs = args.extra_args.into();
    let output = profile.diff(extra_args).await?;

    let mut stdout = std::io::stdout().lock();
    write!(stdout, "{}", output)?;

    Ok(())
}
