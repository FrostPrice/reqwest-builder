use reqwest_builder::{IntoReqwestBuilder, RequestBody};
use serde::Serialize;
use url::Url;

// Test struct with all attribute types
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/api/users/{id}/posts", body = "json")]
struct CompleteTestRequest {
    #[path_param]
    id: u64,

    #[query]
    draft: Option<bool>,

    #[query(name = "include_comments")]
    include_comments: Option<bool>,

    #[header(name = "Authorization")]
    auth_token: String,

    #[header(name = "Content-Type")]
    content_type: String,

    // Body fields
    title: String,
    content: String,
    tags: Vec<String>,
}

// Simple GET request test
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/simple")]
struct SimpleTestRequest {
    #[query]
    page: Option<u32>,
}

// Test with different body types
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/form-test", body = "form")]
struct FormTestRequest {
    username: String,
    password: String,
}

#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "DELETE", path = "/resource/{id}", body = "none")]
struct DeleteTestRequest {
    #[path_param]
    id: u64,

    #[header(name = "Authorization")]
    token: String,
}

#[test]
fn test_complete_derive_macro() {
    let request = CompleteTestRequest {
        id: 123,
        draft: Some(true),
        include_comments: Some(false),
        auth_token: "Bearer test123".to_string(),
        content_type: "application/json".to_string(),
        title: "Test Post".to_string(),
        content: "This is a test post content".to_string(),
        tags: vec!["test".to_string(), "example".to_string()],
    };

    // Test method
    assert_eq!(request.method(), http::Method::POST);

    // Test endpoint with path substitution
    assert_eq!(request.endpoint(), "/api/users/123/posts");

    // Test query parameters
    let query_params = request.query_params().unwrap();
    assert_eq!(query_params.get("draft"), Some(&"true".to_string()));
    assert_eq!(
        query_params.get("include_comments"),
        Some(&"false".to_string())
    );

    // Test headers
    let headers = request.headers().unwrap();
    assert!(headers.auth_token.contains("Bearer test123"));
    assert!(headers.content_type.contains("application/json"));

    // Test body type
    assert_eq!(request.body(), RequestBody::Json);
}

#[test]
fn test_simple_get_request() {
    let request = SimpleTestRequest { page: Some(2) };

    assert_eq!(request.method(), http::Method::GET);
    assert_eq!(request.endpoint(), "/simple");

    let query_params = request.query_params().unwrap();
    assert_eq!(query_params.get("page"), Some(&"2".to_string()));

    // Should not have headers
    assert!(request.headers().is_none());

    assert_eq!(request.body(), RequestBody::Json);
}

#[test]
fn test_form_request() {
    let request = FormTestRequest {
        username: "testuser".to_string(),
        password: "secret123".to_string(),
    };

    assert_eq!(request.method(), http::Method::POST);
    assert_eq!(request.endpoint(), "/form-test");
    assert_eq!(request.body(), RequestBody::Form);
}

#[test]
fn test_delete_request_with_no_body() {
    let request = DeleteTestRequest {
        id: 456,
        token: "Bearer delete123".to_string(),
    };

    assert_eq!(request.method(), http::Method::DELETE);
    assert_eq!(request.endpoint(), "/resource/456");
    assert_eq!(request.body(), RequestBody::None);

    let headers = request.headers().unwrap();
    assert!(headers.token.contains("Bearer delete123"));
}

#[test]
fn test_optional_query_params() {
    let request1 = SimpleTestRequest { page: Some(5) };
    let request2 = SimpleTestRequest { page: None };

    // With Some value
    let params1 = request1.query_params().unwrap();
    assert_eq!(params1.get("page"), Some(&"5".to_string()));

    // With None value
    let params2 = request2.query_params();
    assert!(params2.is_none());
}

#[test]
fn test_into_reqwest_builder() {
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = Url::parse("https://api.example.com").unwrap();

    let request = SimpleTestRequest { page: Some(1) };

    // This should not panic and should return a valid builder
    let builder_result = request.into_reqwest_builder(&client, &base_url);
    assert!(builder_result.is_ok());
}
