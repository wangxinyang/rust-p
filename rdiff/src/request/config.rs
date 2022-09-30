use std::{collections::HashMap, fs::File};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub requests: Vec<Request>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    base: String,
    port: u16,
    method: String,
    param: String,
    req: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config> {
        let args = command::run()?;
        let buf =
            File::open(path).with_context(|| format!("Cannot open file by this path {}", path))?;
        let mut config: Config =
            serde_yaml::from_reader(buf).context("serde_yaml from_reader was error")?;
        for (index, request) in config.requests.iter_mut().enumerate() {
            if let Some(base) = args.base.as_deref() {
                request.base = base.to_string();
            }

            match args.ports.get(index) {
                Some(port) => request.port = port.parse::<u16>()?,
                None => request.port = args.ports.first().unwrap().parse::<u16>()?,
            }

            if !&args.method.is_empty() {
                request.method = args.method.clone();
            }

            if !args.params.is_empty() {
                match args.params.get(index) {
                    Some(param) => request.param = param.to_owned(),
                    None => request.param = args.params.first().unwrap().to_owned(),
                }
            }
        }
        Ok(config)
    }
}

/// get Http response
pub async fn get_response(request: Vec<Request>) -> Result<(String, String)> {
    let mut requests_iter = request.iter();
    let left = requests_iter.next().unwrap();
    let right = requests_iter.next().unwrap();
    let left_res = send_request(left).await?;
    let right_res = send_request(right).await?;
    let left_res_str = left_res.get(&left.req).unwrap().to_owned();
    let right_res_str = right_res.get(&right.req).unwrap().to_owned();
    Ok((left_res_str, right_res_str))
}

/// send Http request
async fn send_request(request: &Request) -> Result<HashMap<String, String>> {
    let path = format!("{}:{}/{}", request.base, request.port, request.param);
    match request.method.as_str() {
        "get" => {
            let resp = reqwest::get(path)
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            Ok(resp)
        }
        _ => Ok(HashMap::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_yaml_param() {
        let config = Config::new("yaml/request.yml").unwrap();
        let path_1 = format!(
            "{}:{}/{}",
            config.requests.first().unwrap().base,
            config.requests.first().unwrap().port,
            config.requests.first().unwrap().param
        );
        assert_eq!(path_1, "http://127.0.0.1:8000/req1");
        assert_eq!(config.requests.first().unwrap().method, "get");

        let path_2 = format!(
            "{}:{}/{}",
            config.requests.last().unwrap().base,
            config.requests.last().unwrap().port,
            config.requests.last().unwrap().param
        );
        assert_eq!(path_2, "http://127.0.0.1:8000/req2");
        assert_eq!(config.requests.last().unwrap().method, "get");
    }
}
