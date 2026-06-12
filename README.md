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
use reloop::{ReloopClient, CreateApiKeyParams};

let reloop = ReloopClient::new("rl_123456789".to_string(), None);

reloop.api_keys().create(CreateApiKeyParams {
    name: "Production key".to_string(),
    enabled: Some(true),
    rate_limit_enabled: Some(true),
}).await?;
```
