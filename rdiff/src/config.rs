use std::collections::HashMap;

use anyhow::{Ok, Result};

use serde::{Deserialize, Serialize};

use crate::{diff_text, ExtraConfigs, RequestProfile};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
    pub fn get_profiles(&self, key: &str) -> Option<&DiffProfile> {
        self.profiles.get(key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    request1: RequestProfile,
    request2: RequestProfile,
    response: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl DiffProfile {
    pub async fn diff(&self, args: ExtraConfigs) -> Result<String> {
        let res1 = self.request1.send(&args).await?;
        let res2 = self.request2.send(&args).await?;

        let text1 = res1.get_text(&self.response).await?;
        let text2 = res2.get_text(&self.response).await?;

        Ok(diff_text(&text1, &text2)?)
    }
}
