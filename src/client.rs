use crate::config::Config;

use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Debug, Default)]
pub struct Client {
    config: Config,
    requester: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        let requester = reqwest::Client::builder().build()?;
        Ok(Self { config, requester })
    }
    pub async fn get(&self, path: &str) -> Result<json::Value, crate::Error> {
        self.send(Method::GET, path, None).await
    }
    pub async fn post(&self, path: &str, body: json::Value) -> Result<json::Value, crate::Error> {
        self.send(Method::POST, path, Some(body)).await
    }
    pub async fn put(&self, path: &str) -> Result<json::Value, crate::Error> {
        self.send(Method::PUT, path, None).await
    }
    pub async fn put_with_body(
        &self,
        path: &str,
        body: json::Value,
    ) -> Result<json::Value, crate::Error> {
        self.send(Method::PUT, path, Some(body)).await
    }
    pub async fn delete(&self, path: &str) -> Result<json::Value, crate::Error> {
        self.send(Method::DELETE, path, None).await
    }
    async fn send(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<json::Value>,
    ) -> Result<json::Value, crate::Error> {
        let url = format!("{}{}", self.config.base_url, path);
        log::debug!("URL: {:?}", url);

        let response = self
            .requester
            .request(method, url)
            .header("x-token", &self.config.token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        log::trace!("Response: {:?}", &text);

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let response: json::Value = json::from_str(&text)?;
                match response.get("data") {
                    Some(_) => {
                        let response: Response = json::from_str(&text)?;
                        Ok(response.data.to_owned())
                    }
                    None => {
                        log::error!("status: {}, body: {:?}", status, text);
                        Err(crate::Error::InvalidArgument(format!(
                            "Incomplete response. Status: {}. Body: {}",
                            status, text
                        )))
                    }
                }
            }
            StatusCode::NOT_FOUND => Err(crate::Error::NotFound("Resource not found".into())),
            _ => {
                log::error!("status: {}, body: {:?}", status, text);
                Err(crate::Error::InvalidArgument(format!(
                    "Request failed. Status: {}. Body: {}",
                    status, text
                )))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    success: bool,
    code: i32,
    data: json::Value,
}
