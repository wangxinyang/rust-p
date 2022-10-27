use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::{
    filter, fmt, prelude::__tracing_subscriber_SubscriberExt, reload, util::SubscriberInitExt,
};

use crate::errors::Errcode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    #[arg(short, long)]
    pub debug: bool,

    /// Directory to mount as root of the container
    #[arg(short = 'm', long = "mount")]
    pub mount_dir: PathBuf,

    /// User ID to create inside the container
    #[arg(short, long)]
    pub uid: u32,

    /// Command to execute inside the container
    #[arg(short, long)]
    pub command: String,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();
    // change log level
    chang_log_level(args.debug);
    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(Errcode::ArgumentInvalid("mount"));
    }
    Ok(args)
}

fn chang_log_level(debug: bool) {
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();
    if debug {
        reload_handle
            .modify(|filter| *filter = filter::LevelFilter::DEBUG)
            .unwrap();
    };
}
