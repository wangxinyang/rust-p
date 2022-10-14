use std::net::SocketAddr;

use serde::Deserialize;
use tokio::fs;
use tracing::log::warn;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub network: NetworkConfig,
    pub features: FeaturesConfig,
}

#[derive(Deserialize, Debug)]
pub struct NetworkConfig {
    addr: String,
    port: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".into(),
            port: "3200".into(),
        }
    }
}

impl From<NetworkConfig> for SocketAddr {
    fn from(network: NetworkConfig) -> Self {
        format!("{}:{}", network.addr, network.port)
            .parse()
            .unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct FeaturesConfig {
    pub font_size: u32,
    pub font_color: String,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            font_size: 2,
            font_color: "#000000".into(),
        }
    }
}

impl Config {
    pub async fn load() -> Self {
        if let Ok(config) = fs::read_to_string("./config/config.yml").await {
            let config: Config = serde_yaml::from_str(&config).unwrap();
            config
        } else {
            warn!("can't load config file, load the default config");
            Self::default()
        }
    }
}
