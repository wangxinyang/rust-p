use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};

use crate::ExtraConfigs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DiffCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
pub enum Commands {
    /// Additional another cmd args
    Add(RunArgs),

    /// Parse user input to create DiffConfig
    Parse,
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Config file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Profile in file
    #[arg(short, long)]
    pub profile: String,

    /// for query params, use `-e key=value`
    /// for headers, use `-e %key=value`
    /// for body,, use `-e @key=value`
    #[arg(short, long, value_parser = parse_extra_args, num_args = 1)]
    pub extra_args: Vec<KeyVal>,
}

#[derive(Debug, Clone)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

#[derive(Debug, Clone)]
enum KeyValType {
    Header,
    Body,
    Query,
}

fn parse_extra_args(env: &str) -> Result<KeyVal> {
    let mut parts = env.splitn(2, '=');
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid  key value pair"))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid  key value pair"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid  key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

/// Vec<KeyVal> -> ExtraConfigs convert
impl From<Vec<KeyVal>> for ExtraConfigs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            body,
            query,
        }
    }
}
