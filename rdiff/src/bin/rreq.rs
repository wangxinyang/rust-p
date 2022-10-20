use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use rdiff::{
    get_body_text, get_header_text, get_status_text, highlighting_text, Commands, DiffCli,
    ExtraConfigs, LoadConfig, RequestConfig, RequestProfile, RunArgs,
};
use std::fmt::Write as _;
use std::io::Write as _;

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
    let config_file = args.config.unwrap_or_else(|| "./rreq.yml".to_string());
    let config: RequestConfig = RequestConfig::load_yaml_config(&config_file).await?;

    let profile = config.get_profiles(&args.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args: ExtraConfigs = args.extra_args.into();
    let url = profile.get_url(&extra_args)?;
    let response = profile.send(&extra_args).await?.into_inner();

    let version = get_status_text(&response)?;
    let headers = get_header_text(&response, &[])?;
    let body = get_body_text(response, &[]).await?;

    let mut output = String::new();
    if atty::is(atty::Stream::Stdout) {
        writeln!(&mut output, "Url: {}\n", url)?;
        write!(&mut output, "{}", version)?;
        write!(&mut output, "{}", highlighting_text(&headers, "yaml")?)?;
        write!(&mut output, "{}", highlighting_text(&body, "json")?)?;
    } else {
        write!(&mut output, "{}", body)?;
    }

    let mut stdout = std::io::stdout().lock();
    write!(stdout, "{}", output)?;

    Ok(())
}

async fn parse() -> Result<()> {
    let theme = ColorfulTheme::default();
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Url")
        .interact_text()?;
    let name: String = Input::with_theme(&theme)
        .with_prompt("Profile name")
        .interact_text()?;
    let request_profile: RequestProfile = url1.parse()?;

    let config = RequestConfig::new(vec![(name, request_profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    let mut stdout = std::io::stdout().lock();
    if atty::is(atty::Stream::Stdout) {
        write!(&mut stdout, "----\n{}", highlighting_text(&result, "yaml")?)?;
    } else {
        write!(&mut stdout, "----\n{}", result)?;
    }

    Ok(())
}
