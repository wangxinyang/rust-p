mod cli;
mod config;
mod req;
mod utils;

pub use self::cli::*;
pub use self::config::*;
pub use self::req::*;
pub use utils::{diff_text, highlighting_text};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExtraConfigs {
    headers: Vec<(String, String)>,
    body: Vec<(String, String)>,
    query: Vec<(String, String)>,
}
