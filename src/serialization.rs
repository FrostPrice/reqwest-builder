use crate::errors::ReqwestBuilderError;
use http::HeaderMap;
use serde::Serialize;
use std::collections::HashMap;

/// Convert a serializable type to form parameters with proper error handling
pub fn serialize_to_form_params<T: Serialize>(
    data: &T,
) -> std::result::Result<HashMap<String, String>, ReqwestBuilderError> {
    let value = serde_json::to_value(data)?;

    let obj = value.as_object().ok_or_else(|| {
        ReqwestBuilderError::SerializationError("Data must serialize to a JSON object".to_string())
    })?;

    let mut params = HashMap::new();
    for (key, val) in obj {
        let value_str = match val {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => continue, // Skip null values
            _ => val.to_string(),                // Arrays and objects as JSON strings
        };
        params.insert(key.clone(), value_str);
    }

    Ok(params)
}

/// Convert serializable headers to HeaderMap with proper error handling
pub fn serialize_to_header_map<T: Serialize>(
    headers: &T,
) -> std::result::Result<HeaderMap, ReqwestBuilderError> {
    let mut header_map = HeaderMap::new();
    let value = serde_json::to_value(headers)?;

    let obj = value.as_object().ok_or_else(|| {
        ReqwestBuilderError::SerializationError(
            "Headers must serialize to a JSON object".to_string(),
        )
    })?;

    for (key, val) in obj {
        if let Some(val_str) = val.as_str() {
            let header_name = http::HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                ReqwestBuilderError::HeaderError {
                    key: key.clone(),
                    value: val_str.to_string(),
                    source: format!("Invalid header name: {}", e),
                }
            })?;

            let header_value = http::HeaderValue::from_str(val_str).map_err(|e| {
                ReqwestBuilderError::HeaderError {
                    key: key.clone(),
                    value: val_str.to_string(),
                    source: format!("Invalid header value: {}", e),
                }
            })?;

            header_map.insert(header_name, header_value);
        } else {
            return Err(ReqwestBuilderError::HeaderError {
                key: key.clone(),
                value: val.to_string(),
                source: "Header value must be a string".to_string(),
            });
        }
    }

    Ok(header_map)
}

/// Construct a URL by combining base URL and endpoint
pub fn construct_url(base_url: &url::Url, endpoint: &str) -> String {
    let base_str = base_url.as_str().trim_end_matches('/');
    let endpoint_str = endpoint.trim_start_matches('/');

    if endpoint_str.is_empty() {
        return base_str.to_string();
    }

    format!("{base_str}/{endpoint_str}")
}
