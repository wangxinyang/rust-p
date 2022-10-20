mod rdiff;
mod rreq;

use crate::ExtraConfigs;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use http::{
    header::{self, HeaderName},
    HeaderMap, HeaderValue, Method,
};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use std::str::FromStr;
use tokio::fs;
use url::Url;

pub use rdiff::{DiffConfig, DiffProfile, ResponseProfile};
pub use rreq::*;

#[async_trait]
pub trait LoadConfig
where
    Self: Sized + DeserializeOwned + ValidateConfig,
{
    async fn load_yaml_config(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yarml(&content)
    }

    fn from_yarml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
}

pub trait ValidateConfig {
    fn validate(&self) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    pub url: Url,

    #[serde(with = "http_serde::method", default)]
    pub method: Method,

    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub params: Option<serde_json::Value>,

    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    pub headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub fn into_inner(self) -> Response {
        self.0
    }

    pub fn get_response_headers(&self) -> Result<Vec<String>> {
        let res_headers = self.0.headers();
        let mut headers = Vec::new();
        for header_item in res_headers.keys() {
            headers.push(header_item.to_string());
        }
        Ok(headers)
    }
}

impl RequestProfile {
    pub fn new(
        url: Url,
        method: Method,
        params: Option<serde_json::Value>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            url,
            method,
            params,
            headers,
            body,
        }
    }

    pub fn get_url(&self, args: &ExtraConfigs) -> Result<String> {
        let (_, query, _) = self.generated(args)?;
        let mut url = self.url.clone();
        if !query.as_object().unwrap().is_empty() {
            let query = serde_qs::to_string(&query)?;
            url.set_query(Some(&query));
        }
        Ok(url.to_string())
    }

    pub async fn send(&self, args: &ExtraConfigs) -> Result<ResponseExt> {
        let (headers, query, body) = self.generated(args)?;
        let client = Client::new();
        let request = client
            .request(self.method.clone(), self.url.clone())
            .headers(headers)
            .body(body)
            .query(&query)
            .build()?;
        let res = client.execute(request).await?;
        Ok(ResponseExt(res))
    }

    fn generated(&self, args: &ExtraConfigs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));
        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        };

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);

        match content_type.as_deref() {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow!("unsupported content type")),
        }
    }

    /// check request parameters of the yaml file
    pub fn validate(&self) -> Result<()> {
        if let Some(params) = &self.params {
            if !params.is_object() {
                return Err(anyhow!(
                    "Params must be an object but got\n{}",
                    serde_yaml::to_string(params)?
                ));
            }
        }
        if let Some(body) = &self.body {
            if !body.is_object() {
                return Err(anyhow!(
                    "Body must be an object but got\n{}",
                    serde_yaml::to_string(body)?
                ));
            }
        }
        Ok(())
    }
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut url = Url::parse(s)?;
        let qs = url.query_pairs();
        let mut params = json!({});
        for (k, v) in qs {
            params[&*k] = v.parse()?;
        }
        url.set_query(None);
        Ok(RequestProfile::new(
            url,
            Method::GET,
            Some(params),
            HeaderMap::new(),
            None,
        ))
    }
}

impl ResponseExt {
    pub async fn get_text(self, profile: &ResponseProfile) -> Result<String> {
        let res = self.0;
        let mut output = get_status_text(&res)?;

        // get header content
        write!(
            &mut output,
            "{}",
            get_header_text(&res, &profile.skip_headers)?
        )?;

        // get body content
        write!(
            &mut output,
            "{}",
            get_body_text(res, &profile.skip_body).await?
        )?;
        Ok(output)
    }
}

pub fn get_status_text(res: &Response) -> Result<String> {
    Ok(format!("{:?} {}\n", res.version(), res.status()))
}

pub fn get_header_text(res: &Response, skip_headers: &[String]) -> Result<String> {
    let mut output = String::new();
    let headers = res.headers();
    for (key, value) in headers.iter() {
        if !skip_headers.iter().any(|sh| sh == key.as_str()) {
            writeln!(&mut output, "{}: {:?}", key, value)?;
        }
    }
    writeln!(&mut output)?;

    Ok(output)
}

pub async fn get_body_text(res: Response, skip_body: &[String]) -> Result<String> {
    // this is must return String not &str, because if return the &str,
    // the &str lifetime as long as res.headers's lifetime
    let content_type = get_content_type(res.headers());

    // casuse the error [move out of `res` occurs here]
    let text = res.text().await?;

    match content_type.as_deref() {
        Some("application/json") => filter_json(&text, skip_body),
        _ => Ok(text),
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next())
        .map(|v| v.to_string())
}

// why Vec<String> -> [String]??
fn filter_json(text: &str, skip_body: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;
    // for now just ignore non-object values
    // in future support array of objects
    if let serde_json::Value::Object(ref mut obj) = json {
        for k in skip_body {
            obj.remove(k);
        }
    }
    Ok(serde_json::to_string_pretty(&json)?)
}

/// if params is empty, skip serialize
fn empty_json_value(v: &Option<serde_json::Value>) -> bool {
    v.as_ref().map_or(true, |v| {
        v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
    })
}
