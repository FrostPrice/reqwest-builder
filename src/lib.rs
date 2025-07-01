use std::{collections::HashMap, path::Path};

use http::HeaderMap;
use serde::{Deserialize, Serialize};
use url::Url;

/// Supported request body types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestBody {
    /// JSON body
    Json,
    /// Form-encoded body
    Form,
    /// Multipart form body (for file uploads, etc.)
    Multipart,
    /// No body (for GET, DELETE, etc.)
    None,
}

/// Query parameters for the request
pub type QueryParams = HashMap<String, String>;

/// Trait for converting request structures into reqwest builders
///
/// This trait provides a standardized way to convert request types into
/// `reqwest_middleware::RequestBuilder` instances with proper configuration.
pub trait IntoReqwestBuilder
where
    Self: Sized + Serialize,
{
    /// Associated type for request headers
    type Headers: Serialize + Clone;

    /// HTTP method for the request
    fn method(&self) -> http::Method;

    /// Endpoint path for the request
    fn endpoint(&self) -> String;

    /// Optional headers for the request
    fn headers(&self) -> Option<Self::Headers> {
        None
    }

    /// Request body type
    fn body(&self) -> RequestBody {
        RequestBody::Json
    }

    /// Optional query parameters
    fn query_params(&self) -> Option<QueryParams> {
        None
    }

    /// Create multipart form - override this for file uploads
    fn create_multipart_form(&self) -> Option<reqwest::multipart::Form> {
        None
    }

    /// Convert the request into a reqwest builder
    ///
    /// This method maintains backward compatibility while providing improved functionality.
    fn into_reqwest_builder(
        self,
        client: &reqwest_middleware::ClientWithMiddleware,
        base_url: &Url,
    ) -> reqwest_middleware::RequestBuilder {
        // Construct URL efficiently
        let url = construct_url_safe(base_url, &self.endpoint());
        let mut builder = client.request(self.method(), &url);

        // Add query parameters if present
        if let Some(params) = self.query_params() {
            builder = builder.query(&params);
        }

        // Handle request body
        builder = self.add_body_to_builder(builder);

        // Add headers if present
        if let Some(headers) = self.headers() {
            let header_map = serialize_to_header_map_safe(&headers);
            builder = builder.headers(header_map);
        }

        builder
    }

    /// Add body to the request builder based on body type
    fn add_body_to_builder(
        &self,
        mut builder: reqwest_middleware::RequestBuilder,
    ) -> reqwest_middleware::RequestBuilder {
        match self.body() {
            RequestBody::Json => {
                // Only add body if it's not empty - improved logic
                if let Ok(json_str) = serde_json::to_string(self) {
                    if json_str != "{}" {
                        builder = builder.json(self);
                    }
                } else {
                    builder = builder.json(self);
                }
            }
            RequestBody::Form => {
                let params = serialize_to_form_params_safe(self);
                builder = builder.form(&params);
            }
            RequestBody::Multipart => {
                if let Some(form) = self.create_multipart_form() {
                    builder = builder.multipart(form);
                }
            }
            RequestBody::None => {
                // No body to add
            }
        }
        builder
    }
}

/// Construct a URL by combining base URL and endpoint
fn construct_url_safe(base_url: &Url, endpoint: &str) -> String {
    let base_str = base_url.as_str().trim_end_matches('/');
    let endpoint_str = endpoint.trim_start_matches('/');

    if endpoint_str.is_empty() {
        return base_str.to_string();
    }

    format!("{base_str}/{endpoint_str}")
}

/// Convert a serializable type to form parameters with improved error handling
fn serialize_to_form_params_safe<T: Serialize>(data: &T) -> HashMap<String, String> {
    serde_json::to_value(data)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .map(|obj| {
            obj.iter()
                .filter_map(|(key, val)| {
                    let value_str = match val {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Null => return None, // Skip null values
                        _ => val.to_string(), // Arrays and objects as JSON strings
                    };
                    Some((key.clone(), value_str))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Convert serializable headers to HeaderMap with improved error handling
fn serialize_to_header_map_safe<T: Serialize>(headers: &T) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    if let Ok(value) = serde_json::to_value(headers)
        && let Some(obj) = value.as_object()
    {
        for (key, val) in obj {
            if let Some(val_str) = val.as_str()
                && let (Ok(header_name), Ok(header_value)) = (
                    http::HeaderName::from_bytes(key.as_bytes()),
                    http::HeaderValue::from_str(val_str),
                )
            {
                header_map.insert(header_name, header_value);
            }
            // Note: Invalid headers are silently skipped for backward compatibility
            // In a future version, we should logging these errors
        }
    }

    header_map
}

/// File data for upload
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUpload {
    pub filename: String,
    #[serde(skip)] // Don't serialize file content
    pub content: Vec<u8>,
    #[serde(skip)] // Don't serialize mime type
    pub mime_type: Option<String>,
}

impl FileUpload {
    /// Create a new file upload from file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let content = std::fs::read(path)?;
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();

        let mime_type = mime_guess::from_path(path)
            .first()
            .map(|mime| mime.to_string());

        Ok(Self {
            filename,
            content,
            mime_type,
        })
    }

    /// Create a new file upload from bytes
    pub fn from_bytes(filename: String, content: Vec<u8>, mime_type: Option<String>) -> Self {
        Self {
            filename,
            content,
            mime_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestRequest {
        field1: String,
        field2: i32,
        field3: Option<String>,
    }

    impl IntoReqwestBuilder for TestRequest {
        type Headers = ();

        fn method(&self) -> http::Method {
            http::Method::POST
        }

        fn endpoint(&self) -> String {
            "/test/endpoint".to_string()
        }
    }

    #[test]
    fn test_construct_url_safe() {
        let base_url = Url::parse("https://api.example.com/").unwrap();
        let result = construct_url_safe(&base_url, "/test/endpoint");
        assert_eq!(result, "https://api.example.com/test/endpoint");

        let base_url = Url::parse("https://api.example.com").unwrap();
        let result = construct_url_safe(&base_url, "test/endpoint");
        assert_eq!(result, "https://api.example.com/test/endpoint");

        let base_url = Url::parse("https://api.example.com").unwrap();
        let result = construct_url_safe(&base_url, "");
        assert_eq!(result, "https://api.example.com");
    }

    #[test]
    fn test_serialize_to_form_params_safe() {
        let test_data = TestRequest {
            field1: "value1".to_string(),
            field2: 42,
            field3: Some("value3".to_string()),
        };

        let params = serialize_to_form_params_safe(&test_data);
        assert_eq!(params.get("field1"), Some(&"value1".to_string()));
        assert_eq!(params.get("field2"), Some(&"42".to_string()));
        assert_eq!(params.get("field3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_serialize_to_form_params_safe_with_null() {
        let test_data = TestRequest {
            field1: "value1".to_string(),
            field2: 42,
            field3: None,
        };

        let params = serialize_to_form_params_safe(&test_data);
        assert_eq!(params.get("field1"), Some(&"value1".to_string()));
        assert_eq!(params.get("field2"), Some(&"42".to_string()));
        assert_eq!(params.get("field3"), None); // Should be skipped
    }

    #[test]
    fn test_serialize_to_header_map_safe() {
        #[derive(Serialize)]
        struct TestHeaders {
            #[serde(rename = "Content-Type")]
            content_type: String,
            #[serde(rename = "Authorization")]
            authorization: String,
        }

        let headers = TestHeaders {
            content_type: "application/json".to_string(),
            authorization: "Bearer token123".to_string(),
        };

        let header_map = serialize_to_header_map_safe(&headers);
        assert_eq!(header_map.get("Content-Type").unwrap(), "application/json");
        assert_eq!(header_map.get("Authorization").unwrap(), "Bearer token123");
    }

    #[test]
    fn test_request_body_none() {
        #[derive(Serialize)]
        struct GetRequest {
            id: String,
        }

        impl IntoReqwestBuilder for GetRequest {
            type Headers = ();

            fn method(&self) -> http::Method {
                http::Method::GET
            }

            fn endpoint(&self) -> String {
                format!("/users/{}", self.id)
            }

            fn body(&self) -> RequestBody {
                RequestBody::None
            }
        }

        let request = GetRequest {
            id: "123".to_string(),
        };

        // This should not panic and should handle the None body type correctly
        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let base_url = Url::parse("https://api.example.com").unwrap();
        let _builder = request.into_reqwest_builder(&client, &base_url);
    }
}
