# Reloop Rust SDK

Official Rust client for the Reloop API.

## Install

```toml
[dependencies]
reloop = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## Usage

```rust
use reloop::ReloopClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reloop = ReloopClient::new("re_123456789".to_string(), None);

    reloop.contacts().create(json!({
        "email": "john.doe@example.com",
        "first_name": "John",
        "last_name": "Doe",
        "unsubscribed": false,
    })).await?;

    Ok(())
}
```

## API Keys

```rust
use reloop::{ReloopClient, CreateApiKeyParams, ApiKeyListParams};

let reloop = ReloopClient::new("rl_123456789".to_string(), None);

reloop.api_keys().create(CreateApiKeyParams {
    name: "Production key".to_string(),
    enabled: Some(true),
    rate_limit_enabled: Some(true),
}).await?;

reloop.api_keys().list(Some(ApiKeyListParams {
    page: Some(1),
    limit: Some(10),
    ..Default::default()
})).await?;

reloop.api_keys().rotate("key_123456789").await?;
reloop.api_keys().pause("key_123456789").await?;
reloop.api_keys().enable("key_123456789").await?;
```

## Domains

```rust
use reloop::{
    ReloopClient, CreateDomainParams, DomainTlsMode, ForwardDnsParams, ListDomainsParams,
    UpdateDomainParams,
};

let reloop = ReloopClient::new("rl_123456789".to_string(), None);

let domain = reloop.domain().create(CreateDomainParams {
    domain: "send.example.com".to_string(),
    custom_return_path: Some("inbound".to_string()),
    tracking: None,
    click_tracking: Some(true),
    open_tracking: Some(true),
    tls: Some(DomainTlsMode::Opportunistic),
    sending_email: Some(true),
    receiving_email: Some(true),
}).await?;

let domains = reloop.domain().list(Some(ListDomainsParams {
    page: Some(1),
    limit: Some(10),
    q: None,
    status: None,
})).await?;

reloop.domain().update(
    &domain.id,
    UpdateDomainParams {
        click_tracking: Some(false),
        open_tracking: None,
        sending_email: Some(true),
        receiving_email: None,
        tls: None,
    },
).await?;

reloop.domain().verify(&domain.id).await?;
reloop.domain().forward_dns(
    &domain.id,
    ForwardDnsParams { email: "admin@example.com".to_string() },
).await?;
let nameservers = reloop.domain().get_nameservers(&domain.id).await?;
reloop.domain().delete(&domain.id).await?;
```
