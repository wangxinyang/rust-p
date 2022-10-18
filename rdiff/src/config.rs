use std::collections::HashMap;

use anyhow::{Ok, Result};
use http::{HeaderMap, Method};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::ExtraConfigs;

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
pub struct RequestProfile {
    url: Url,

    #[serde(with = "http_serde::method", default)]
    method: Method,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    params: Option<serde_json::Value>,

    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skip_headers: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skip_body: Vec<String>,
}

impl DiffProfile {
    pub async fn diff(&self, _args: ExtraConfigs) -> Result<()> {
        println!("{:?}", self);
        println!("{:?}", _args);
        Ok(())
    }
}
