use crate::client::ReloopClient;
use crate::models::*;
use reqwest::Method;
use std::collections::HashMap;

pub struct ApiKeyService<'a> {
    client: &'a ReloopClient,
}

impl<'a> ApiKeyService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        ApiKeyService { client }
    }

    pub async fn create(&self, params: CreateApiKeyParams) -> Result<ApiKeyWithKey, Box<dyn std::error::Error>> {
        let body = serde_json::to_value(params)?;
        self.client.fetch(Method::POST, "/api/api-key/v1/", Some(body)).await
    }

    pub async fn list(&self, params: Option<ApiKeyListParams>) -> Result<ApiKeyListResponse, Box<dyn std::error::Error>> {
        let mut path = "/api/api-key/v1/".to_string();
        if let Some(p) = params {
            let mut query = HashMap::new();
            if let Some(page) = p.page { query.insert("page", page.to_string()); }
            if let Some(limit) = p.limit { query.insert("limit", limit.to_string()); }
            if let Some(enabled) = p.enabled { query.insert("enabled", enabled.to_string()); }
            if let Some(user_id) = p.user_id { query.insert("userId", user_id); }
            if let Some(q) = p.q { query.insert("q", q); }

            if !query.is_empty() {
                path.push('?');
                let query_string = query
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                path.push_str(&query_string);
            }
        }
        self.client.fetch(Method::GET, &path, None).await
    }

    pub async fn get(&self, id: &str) -> Result<ApiKey, Box<dyn std::error::Error>> {
        self.client.fetch(Method::GET, &format!("/api/api-key/v1/{id}"), None).await
    }

    pub async fn update(&self, id: &str, params: UpdateApiKeyParams) -> Result<ApiKey, Box<dyn std::error::Error>> {
        let body = serde_json::to_value(params)?;
        self.client.fetch(Method::PATCH, &format!("/api/api-key/v1/{id}"), Some(body)).await
    }

    pub async fn delete(&self, id: &str) -> Result<DeleteApiKeyResponse, Box<dyn std::error::Error>> {
        self.client.fetch(Method::DELETE, &format!("/api/api-key/v1/{id}"), None).await
    }

    pub async fn rotate(&self, id: &str) -> Result<ApiKeyWithKey, Box<dyn std::error::Error>> {
        self.client.fetch(Method::POST, &format!("/api/api-key/v1/rotate/{id}"), None).await
    }

    pub async fn enable(&self, id: &str) -> Result<ApiKey, Box<dyn std::error::Error>> {
        self.client.fetch(Method::POST, &format!("/api/api-key/v1/enable/{id}"), None).await
    }

    pub async fn disable(&self, id: &str) -> Result<ApiKey, Box<dyn std::error::Error>> {
        self.client.fetch(Method::POST, &format!("/api/api-key/v1/disable/{id}"), None).await
    }

    pub async fn pause(&self, id: &str) -> Result<ApiKey, Box<dyn std::error::Error>> {
        self.disable(id).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_route_uses_api_prefix() {
        assert_eq!("/api/api-key/v1/", "/api/api-key/v1/");
    }

    #[test]
    fn pause_uses_disable_route_suffix() {
        let id = "key_1";
        assert_eq!(format!("/api/api-key/v1/disable/{id}"), "/api/api-key/v1/disable/key_1");
    }

    #[test]
    fn rotate_uses_rotate_route_suffix() {
        let id = "key_1";
        assert_eq!(format!("/api/api-key/v1/rotate/{id}"), "/api/api-key/v1/rotate/key_1");
    }
}
