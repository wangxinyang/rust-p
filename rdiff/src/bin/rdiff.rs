use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use rdiff::{
    highlighting_text, Commands, DiffCli, DiffConfig, DiffProfile, ExtraConfigs, LoadConfig,
    RequestProfile, ResponseProfile, RunArgs,
};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    // get config
    let config = DiffCli::parse();
    match config.command {
        Commands::Add(args) => run(args).await?,
        Commands::Parse => parse().await?,
        _ => panic!("Invalid command"),
    };
    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./rdiff.yml".to_string());
    let config: DiffConfig = DiffConfig::load_yaml_config(&config_file).await?;

    let profile = config.get_profiles(&args.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args: ExtraConfigs = args.extra_args.into();
    let output = profile.diff(extra_args).await?;

    let mut stdout = std::io::stdout().lock();
    write!(stdout, "{}", output)?;

    Ok(())
}

/// parse the content of user command input
async fn parse() -> Result<()> {
    let theme = ColorfulTheme::default();
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Request1")
        .interact_text()?;
    let url2: String = Input::with_theme(&theme)
        .with_prompt("Request2")
        .interact_text()?;
    let name: String = Input::with_theme(&theme)
        .with_prompt("Profile name")
        .interact_text()?;
    let request1: RequestProfile = url1.parse()?;
    let request2: RequestProfile = url2.parse()?;
    // first send request, get the headers, put them in skip_header
    let res = request1.send(&ExtraConfigs::default()).await?;
    let select_skip_header = res.get_response_headers()?;
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&select_skip_header)
        .interact()?;
    let skip_headers: Vec<String> = chosen
        .iter()
        .map(|x| select_skip_header[*x].to_string())
        .collect();

    let response = ResponseProfile::new(skip_headers, vec![]);
    let diff_profile = DiffProfile::new(request1, request2, response);
    let config = DiffConfig::new(vec![(name, diff_profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    let mut stdout = std::io::stdout().lock();
    write!(&mut stdout, "----\n{}", highlighting_text(&result, "yaml")?)?;
    Ok(())
}
