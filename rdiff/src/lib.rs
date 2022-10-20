mod cli;
mod config;
mod utils;

pub use self::cli::*;
pub use config::{
    get_body_text, get_header_text, get_status_text, DiffConfig, DiffProfile, LoadConfig,
    RequestConfig, RequestProfile, ResponseProfile,
};
pub use utils::{diff_text, highlighting_text};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExtraConfigs {
    headers: Vec<(String, String)>,
    body: Vec<(String, String)>,
    query: Vec<(String, String)>,
}
