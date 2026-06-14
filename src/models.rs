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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DomainStatus {
    Pending,
    Verifying,
    Active,
    Suspended,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DomainTlsMode {
    Opportunistic,
    Enforced,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DnsRecordPurpose {
    Sending,
    Receiving,
    Tracking,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    pub id: String,
    pub record_type: String,
    pub record_type_name: String,
    pub domain: String,
    pub name: String,
    pub value: String,
    pub ttl: String,
    pub priority: Option<i32>,
    pub verification_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<DnsRecordPurpose>,
    pub created_at: String,
    pub status: DomainStatus,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub object: String,
    pub id: String,
    pub domain: String,
    pub status: DomainStatus,
    pub user_verified_domain: bool,
    pub system_verified: bool,
    pub custom_return_path: String,
    pub tracking_subdomain: String,
    pub is_click_tracking_enabled: bool,
    pub is_open_tracking_enabled: bool,
    pub tls: DomainTlsMode,
    pub is_tracking_domain: bool,
    pub is_sending_email_enabled: bool,
    pub is_receiving_email_enabled: bool,
    pub verification_failed_reason: Option<String>,
    pub dns_records: Vec<DnsRecord>,
    pub last_verified_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDomainParams {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_return_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<DomainTlsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiving_email: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UpdateDomainParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiving_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<DomainTlsMode>,
}

#[derive(Debug, Default)]
pub struct ListDomainsParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub q: Option<String>,
    pub status: Option<DomainStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainListResponse {
    pub object: String,
    pub domains: Vec<Domain>,
    pub total: i32,
    pub page: i32,
    pub limit: i32,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainStatusResponse {
    pub id: String,
    pub status: DomainStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ForwardDnsParams {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardDnsResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainNameserversResponse {
    pub object: String,
    pub domain_id: String,
    pub domain: String,
    pub nameservers: Option<Vec<String>>,
    pub dns_provider: Option<String>,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMailResponse {
    pub success: bool,
    pub message_id: String,
    pub status: String,
    pub timestamp: String,
    pub id: String,
}

#[cfg(test)]
mod domain_model_tests {
    use super::*;

    #[test]
    fn create_domain_params_serialize_with_snake_case() {
        let json = serde_json::to_string(&CreateDomainParams {
            domain: "send.example.com".to_string(),
            custom_return_path: Some("inbound".to_string()),
            tracking: None,
            click_tracking: Some(true),
            open_tracking: None,
            tls: Some(DomainTlsMode::Opportunistic),
            sending_email: Some(true),
            receiving_email: None,
        })
        .expect("serialize create params");

        assert!(json.contains("\"click_tracking\":true"));
        assert!(json.contains("\"custom_return_path\":\"inbound\""));
        assert!(!json.contains("clickTracking"));
    }

    #[test]
    fn update_domain_params_serialize_with_snake_case() {
        let json = serde_json::to_string(&UpdateDomainParams {
            click_tracking: Some(false),
            open_tracking: Some(true),
            sending_email: None,
            receiving_email: None,
            tls: Some(DomainTlsMode::Enforced),
        })
        .expect("serialize update params");

        assert!(json.contains("\"click_tracking\":false"));
        assert!(!json.contains("clickTracking"));
    }
}
