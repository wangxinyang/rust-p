use super::{LoadConfig, ValidateConfig};
use crate::RequestProfile;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestConfig {
    #[serde(flatten)]
    profiles: HashMap<String, RequestProfile>,
}

impl RequestConfig {
    pub fn new(profiles: HashMap<String, RequestProfile>) -> Self {
        Self { profiles }
    }

    pub fn get_profiles(&self, key: &str) -> Option<&RequestProfile> {
        self.profiles.get(key)
    }
}

impl LoadConfig for RequestConfig {}

impl ValidateConfig for RequestConfig {
    fn validate(&self) -> Result<()> {
        for (key, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("failed to validate profile: {}", key))?;
        }
        Ok(())
    }
}
