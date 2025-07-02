# Reqwest Builder

A builder for reqwest requests with support for custom headers, query parameters, and body content, featuring comprehensive error handling.

## Features

- ✅ **Builder Pattern**: Trait-based approach for converting request structures into reqwest builders
- ✅ **Multiple Body Types**: Support for JSON, form-encoded, multipart, and no-body requests
- ✅ **Error Handling**: Comprehensive error handling with detailed error messages
- ✅ **File Uploads**: Built-in support for file uploads with MIME type detection
- ✅ **Header Management**: Safe header serialization with proper error reporting
- ✅ **Backward Compatibility**: Maintains compatibility with existing code
- ✅ **Modular Architecture**: Clean separation of concerns with well-organized modules

## Error Handling

The library provides two approaches for error handling:

### 1. Backward Compatible (Silent Failures)

```rust
use reqwest_builder::{IntoReqwestBuilder, RequestBody};

// This method silently skips invalid headers/data
let builder = request.into_reqwest_builder(&client, &base_url);
```

### 2. Explicit Error Handling (Recommended)

```rust
use reqwest_builder::{IntoReqwestBuilder, ReqwestBuilderError};

// This method returns detailed errors
match request.try_into_reqwest_builder(&client, &base_url) {
    Ok(builder) => {
        // Use the builder
    }
    Err(ReqwestBuilderError::HeaderError { key, value, source }) => {
        eprintln!("Invalid header '{}': '{}' - {}", key, value, source);
    }
    Err(ReqwestBuilderError::SerializationError(msg)) => {
        eprintln!("Serialization error: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Error Types

The library provides detailed error information through the `ReqwestBuilderError` enum:

- `SerializationError`: Issues with JSON serialization
- `HeaderError`: Invalid header names or values
- `UrlError`: URL construction problems
- `IoError`: File I/O errors
- `InvalidRequest`: General request configuration issues

## Usage Example

```rust
use reqwest_builder::{IntoReqwestBuilder, RequestBody, FileUpload};
use serde::Serialize;

#[derive(Serialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct AuthHeaders {
    #[serde(rename = "Authorization")]
    authorization: String,
    #[serde(rename = "Content-Type")]
    content_type: String,
}

impl IntoReqwestBuilder for CreateUserRequest {
    type Headers = AuthHeaders;

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> String {
        "/users".to_string()
    }

    fn headers(&self) -> Option<Self::Headers> {
        Some(AuthHeaders {
            authorization: "Bearer token123".to_string(),
            content_type: "application/json".to_string(),
        })
    }

    fn body(&self) -> RequestBody {
        RequestBody::Json
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = url::Url::parse("https://api.example.com")?;

    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    // Use the new error-handling method
    let builder = request.try_into_reqwest_builder(&client, &base_url)?;
    let response = builder.send().await?;

    println!("Status: {}", response.status());
    Ok(())
}
```

## File Upload Example

```rust
use reqwest_builder::{FileUpload, IntoReqwestBuilder, RequestBody};

// Create file upload with error handling
let file = FileUpload::from_path("document.pdf")?;

// Or create from bytes
let file = FileUpload::from_bytes(
    "data.json".to_string(),
    b"{}".to_vec(),
    Some("application/json".to_string())
);
```
