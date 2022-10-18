mod cli;
mod config;

pub use self::cli::*;
pub use self::config::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraConfigs {
    headers: Vec<(String, String)>,
    body: Vec<(String, String)>,
    query: Vec<(String, String)>,
}
