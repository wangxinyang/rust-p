use anyhow::Result;
use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "Diff Http Response Content By Rust",
    author = "wxyang",
    version = "0.1.0",
    about = "diff http response from two request",
    long_about = "Rust response diff"
)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(value_name = "BASE", short = 'b', long = "base", help = "url base")]
    pub base: Option<String>,

    #[arg(
        value_name = "PORTS",
        short = 'p',
        long = "ports",
        help = "url port",
        default_value = "8100",
        num_args = 1..
    )]
    pub ports: Vec<String>,

    #[arg(
        value_name = "METHOD",
        short = 'm',
        long = "method",
        default_value = "get",
        help = "http method"
    )]
    pub method: String,

    #[arg(
        value_name = "PARAMS",
        short = 'c',
        long = "params",
        help = "http params",
        num_args = 1..
    )]
    pub params: Vec<String>,
}

pub fn run() -> Result<Cli> {
    let args = Cli::parse();
    Ok(args)
}
