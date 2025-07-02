use reqwest_builder::{IntoReqwestBuilder, RequestBody, errors::ReqwestBuilderError};
use serde::Serialize;
use url::Url;

#[derive(Serialize, Clone)]
struct ApiRequest {
    name: String,
    email: String,
}

#[derive(Serialize, Clone)]
struct InvalidHeaders {
    #[serde(rename = "Invalid\r\nHeader")]
    invalid_header: String,
}

impl IntoReqwestBuilder for ApiRequest {
    type Headers = InvalidHeaders;

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> String {
        "/users".to_string()
    }

    fn headers(&self) -> Option<Self::Headers> {
        Some(InvalidHeaders {
            invalid_header: "test".to_string(),
        })
    }

    fn body(&self) -> RequestBody {
        RequestBody::Json
    }
}

fn main() {
    let request = ApiRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = Url::parse("https://api.example.com").unwrap();

    // Using error-handling method
    println!("\n=== Using error-handling method ===");
    match request.into_reqwest_builder(&client, &base_url) {
        Ok(_builder) => {
            println!("Builder created successfully");
        }
        Err(ReqwestBuilderError::HeaderError { key, value, source }) => {
            println!("Header error detected!");
            println!("  Key: {}", key);
            println!("  Value: {}", value);
            println!("  Source: {}", source);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Demonstrate file upload error handling
    println!("\n=== File upload error handling ===");
    match reqwest_builder::FileUpload::from_path("/non/existent/file.txt") {
        Ok(_) => println!("File loaded successfully"),
        Err(ReqwestBuilderError::IoError(msg)) => {
            println!("File I/O error: {}", msg);
        }
        Err(e) => println!("Other error: {}", e),
    }
}
