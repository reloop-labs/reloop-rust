use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::services::{api_key::ApiKeyService, contacts::ContactsService, domain::DomainService, mail::MailService};

pub struct ReloopClient {
    api_key: String,
    base_url: String,
    http_client: Client,
}

impl ReloopClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        ReloopClient {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://reloop.sh".to_string()),
            http_client: Client::new(),
        }
    }

    pub fn api_keys(&self) -> ApiKeyService<'_> {
        ApiKeyService::new(self)
    }

    pub fn contacts(&self) -> ContactsService<'_> {
        ContactsService::new(self)
    }

    pub fn domain(&self) -> DomainService<'_> {
        DomainService::new(self)
    }

    pub fn mail(&self) -> MailService<'_> {
        MailService::new(self)
    }

    pub async fn fetch_value(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.fetch(method, path, body).await
    }

    pub async fn fetch<T>(&self, method: Method, path: &str, body: Option<Value>) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http_client.request(method, url)
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let response = builder.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Reloop API Error: {}", error_text).into());
        }

        if response.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::from_value(serde_json::Value::Null)?);
        }

        let data = response.json::<T>().await?;
        Ok(data)
    }
}
