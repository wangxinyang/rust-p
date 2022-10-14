use std::collections::HashMap;

use http::{HeaderMap, Method};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    profiles: HashMap<String, DiffProfile>,
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
