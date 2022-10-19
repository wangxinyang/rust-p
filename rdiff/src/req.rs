use anyhow::{anyhow, Result};
use http::{
    header::{self, HeaderName},
    HeaderMap, HeaderValue, Method,
};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use std::str::FromStr;
use url::Url;

use crate::{ExtraConfigs, ResponseProfile};

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

pub struct ResponseExt(Response);

impl RequestProfile {
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

    pub fn generated(&self, args: &ExtraConfigs) -> Result<(HeaderMap, serde_json::Value, String)> {
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
}

impl ResponseExt {
    pub async fn get_text(self, profile: &ResponseProfile) -> Result<String> {
        let res = self.0;
        // get header content
        let mut output = get_header_text(&res, &profile.skip_headers)?;

        // this is must return String not &str, because if return the &str,
        // the &str lifetime as long as res.headers's lifetime
        let content_type = get_content_type(res.headers());

        // casuse the error [move out of `res` occurs here]
        let text = res.text().await?;

        match content_type.as_deref() {
            Some("application/json") => {
                let text = filter_json(&text, &profile.skip_body)?;
                writeln!(&mut output, "{}", &text)?;
            }
            _ => writeln!(&mut output, "{}", &text)?,
        }
        Ok(output)
    }
}

fn get_header_text(res: &Response, skip_headers: &[String]) -> Result<String> {
    let mut output = String::new();
    writeln!(&mut output, "{:?} {}", res.version(), res.status())?;
    let headers = res.headers();
    for (key, value) in headers.iter() {
        if !skip_headers.iter().any(|sh| sh == key.as_str()) {
            output.push_str(&format!("{}: {:?}", key, value));
        }
    }
    writeln!(&mut output)?;

    Ok(output)
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
