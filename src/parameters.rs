use serde_json::{Map, Value};

const REQUEST_KEY_MAP: &[(&str, &str)] = &[
    ("first_name", "firstName"),
    ("last_name", "lastName"),
    ("group_ids", "groupIds"),
    ("group_id", "groupId"),
    ("fallback_value", "fallbackValue"),
    ("default_subscription", "defaultSubscription"),
    ("channel_id", "channelId"),
    ("property_name", "propertyName"),
    ("property_type", "propertyType"),
    ("contact_id", "contactId"),
    ("rate_limit_enabled", "rateLimitEnabled"),
    ("user_id", "userId"),
];

pub fn for_request(parameters: Value) -> Value {
    match parameters {
        Value::Object(map) => Value::Object(for_request_map(map)),
        other => other,
    }
}

pub fn for_query(options: Value) -> Value {
    for_request(options)
}

fn for_request_map(map: Map<String, Value>) -> Map<String, Value> {
    let mut normalized = Map::new();

    for (key, value) in map.iter() {
        if key == "unsubscribed" {
            if !map.contains_key("status") {
                if let Some(unsubscribed) = value.as_bool() {
                    normalized.insert(
                        "status".to_string(),
                        Value::String(if unsubscribed {
                            "unsubscribed".to_string()
                        } else {
                            "subscribed".to_string()
                        }),
                    );
                }
            }
            continue;
        }

        let api_key = map_request_key(key);
        normalized.insert(api_key, normalize_value(value.clone(), true));
    }

    normalized
}

fn normalize_value(value: Value, is_request: bool) -> Value {
    match value {
        Value::Object(map) => {
            if is_request {
                Value::Object(for_request_map(map))
            } else {
                Value::Object(map)
            }
        }
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| normalize_value(item, is_request))
                .collect(),
        ),
        other => other,
    }
}

fn map_request_key(key: &str) -> String {
    REQUEST_KEY_MAP
        .iter()
        .find(|(from, _)| *from == key)
        .map(|(_, to)| (*to).to_string())
        .unwrap_or_else(|| to_camel_case(key))
}

fn to_camel_case(key: &str) -> String {
    if !key.contains('_') {
        return key.to_string();
    }

    let mut parts = key.split('_');
    let first = parts.next().unwrap_or_default();
    let mut result = first.to_string();

    for part in parts {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        if let Some(first_char) = chars.next() {
            result.push(first_char.to_ascii_uppercase());
            result.push_str(chars.as_str());
        }
    }

    result
}

pub fn build_query(options: &Value) -> String {
    let Some(map) = options.as_object() else {
        return String::new();
    };

    if map.is_empty() {
        return String::new();
    }

    let query = map
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

    format!("?{query}")
}
