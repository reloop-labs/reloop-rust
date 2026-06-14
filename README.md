# Reloop Rust SDK

## Before you send

You need two things:

1. **API key** — create one in your Reloop account
2. **Verified domain** — add and verify a sending domain; use it in the `from` address

For setup details and the full API reference, see [reloop.sh/docs](https://reloop.sh/docs).

## Send email

```toml
[dependencies]
reloop = "1.8.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

```rust
use reloop::ReloopClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reloop = ReloopClient::new("rl_your_api_key_here".to_string(), None);

    let result = reloop.mail().send(json!({
        "from": "Reloop <hello@your-verified-domain.com>",
        "to": "user@example.com",
        "subject": "Welcome to Reloop",
        "html": "<p>Thanks for signing up.</p>",
        "text": "Thanks for signing up.",
    })).await?;

    println!("{} {}", result.message_id, result.id);
    Ok(())
}
```

More examples and optional fields: [reloop.sh/docs](https://reloop.sh/docs)
