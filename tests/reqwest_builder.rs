use reqwest_builder::{
    construct_url,
    errors::ReqwestBuilderError,
    file_upload::FileUpload,
    serialization::{serialize_to_form_params, serialize_to_header_map},
    trait_impl::IntoReqwestBuilder,
    types::RequestBody,
};
use serde::Serialize;
use url::Url;

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
fn test_construct_url() {
    let base_url = Url::parse("https://api.example.com/").unwrap();
    let result = construct_url(&base_url, "/test/endpoint");
    assert_eq!(result, "https://api.example.com/test/endpoint");

    let base_url = Url::parse("https://api.example.com").unwrap();
    let result = construct_url(&base_url, "test/endpoint");
    assert_eq!(result, "https://api.example.com/test/endpoint");

    let base_url = Url::parse("https://api.example.com").unwrap();
    let result = construct_url(&base_url, "");
    assert_eq!(result, "https://api.example.com");
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

#[test]
fn test_serialize_to_header_map_with_error_handling() {
    #[derive(Serialize)]
    struct ValidHeaders {
        #[serde(rename = "Content-Type")]
        content_type: String,
        #[serde(rename = "Authorization")]
        authorization: String,
    }

    let headers = ValidHeaders {
        content_type: "application/json".to_string(),
        authorization: "Bearer token123".to_string(),
    };

    let result = serialize_to_header_map(&headers);
    assert!(result.is_ok());
    let header_map = result.unwrap();
    assert_eq!(header_map.get("Content-Type").unwrap(), "application/json");
    assert_eq!(header_map.get("Authorization").unwrap(), "Bearer token123");
}

#[test]
fn test_serialize_to_header_map_invalid_header() {
    #[derive(Serialize)]
    struct InvalidHeaders {
        #[serde(rename = "Invalid\r\nHeader")]
        invalid_header: String,
    }

    let headers = InvalidHeaders {
        invalid_header: "value".to_string(),
    };

    let result = serialize_to_header_map(&headers);
    assert!(result.is_err());
    match result.unwrap_err() {
        ReqwestBuilderError::HeaderError { key, .. } => {
            assert_eq!(key, "Invalid\r\nHeader");
        }
        _ => panic!("Expected HeaderError"),
    }
}

#[test]
fn test_serialize_to_form_params_with_error_handling() {
    let test_data = TestRequest {
        field1: "value1".to_string(),
        field2: 42,
        field3: Some("value3".to_string()),
    };

    let result = serialize_to_form_params(&test_data);
    assert!(result.is_ok());
    let params = result.unwrap();
    assert_eq!(params.get("field1"), Some(&"value1".to_string()));
    assert_eq!(params.get("field2"), Some(&"42".to_string()));
    assert_eq!(params.get("field3"), Some(&"value3".to_string()));
}

#[test]
fn test_file_upload_error_handling() {
    // Test with non-existent file
    let result = FileUpload::from_path("/non/existent/file.txt");
    assert!(result.is_err());
    match result.unwrap_err() {
        ReqwestBuilderError::IoError(_) => {
            // Expected
        }
        _ => panic!("Expected IoError"),
    }
}

#[test]
fn test_into_reqwest_builder() {
    let request = TestRequest {
        field1: "value1".to_string(),
        field2: 42,
        field3: Some("value3".to_string()),
    };

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = Url::parse("https://api.example.com").unwrap();

    let result = request.into_reqwest_builder(&client, &base_url);
    assert!(result.is_ok());
}
