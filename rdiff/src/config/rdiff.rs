use super::{LoadConfig, ValidateConfig};
use crate::{diff_text, ExtraConfigs, RequestProfile};
use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    profiles: HashMap<String, DiffProfile>,
}

impl LoadConfig for DiffConfig {}

impl ValidateConfig for DiffConfig {
    fn validate(&self) -> Result<()> {
        for (key, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("failed to validate profile: {}", key))?;
        }
        Ok(())
    }
}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { profiles }
    }

    pub fn get_profiles(&self, key: &str) -> Option<&DiffProfile> {
        self.profiles.get(key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    request1: RequestProfile,
    request2: RequestProfile,
    #[serde(skip_serializing_if = "is_default", default)]
    response: ResponseProfile,
}

fn is_default<T: Default + PartialEq>(v: &T) -> bool {
    v == &T::default()
}

impl DiffProfile {
    pub fn new(
        request1: RequestProfile,
        request2: RequestProfile,
        response: ResponseProfile,
    ) -> Self {
        Self {
            request1,
            request2,
            response,
        }
    }

    pub async fn diff(&self, args: ExtraConfigs) -> Result<String> {
        let res1 = self.request1.send(&args).await?;
        let res2 = self.request2.send(&args).await?;

        let text1 = res1.get_text(&self.response).await?;
        let text2 = res2.get_text(&self.response).await?;

        diff_text(&text1, &text2)
    }
}

impl ValidateConfig for DiffProfile {
    fn validate(&self) -> Result<()> {
        self.request1
            .validate()
            .context("request1 failed to validate")?;
        self.request2
            .validate()
            .context("request2 failed to validate")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self {
            skip_headers,
            skip_body,
        }
    }
}
