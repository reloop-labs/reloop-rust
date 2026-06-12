use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub image: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    pub name: Option<String>,
    pub start: Option<String>,
    pub prefix: Option<String>,
    pub organization_id: String,
    pub user_id: String,
    pub refill_interval: Option<i32>,
    pub refill_amount: Option<i32>,
    pub last_refill_at: Option<String>,
    pub enabled: bool,
    pub rate_limit_enabled: bool,
    pub rate_limit_time_window: i32,
    pub rate_limit_max: i32,
    pub request_count: i32,
    pub remaining: Option<i32>,
    pub last_request: Option<String>,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Option<String>,
    pub metadata: Option<String>,
    pub created_by: Option<User>,
    pub object: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyWithKey {
    pub id: String,
    pub name: Option<String>,
    pub key: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Option<String>,
    pub object: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyListResponse {
    pub object: String,
    pub api_keys: Vec<ApiKey>,
    pub total: i32,
    pub page: i32,
    pub limit: i32,
    pub event: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ApiKeyListParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub enabled: Option<bool>,
    pub user_id: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiKeyResponse {
    pub id: String,
    pub message: String,
    pub object: String,
    pub event: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
