# Reqwest Builder Derive

A derive macro for the `reqwest-builder` crate that automatically generates `IntoReqwestBuilder` implementations for your structs.

## Overview

This crate provides the `IntoReqwestBuilder` derive macro that automatically implements the trait for your request structs, eliminating the need for manual implementation. Simply annotate your struct with attributes to define HTTP request properties.

## Features

- **Automatic Implementation**: Derive `IntoReqwestBuilder` trait automatically
- **Flexible Attributes**: Control every aspect of your HTTP request through attributes
- **Path Parameters**: Replace placeholders in URLs with struct fields
- **Query Parameters**: Automatically handle query string generation
- **Custom Headers**: Define headers with custom names
- **Multiple Body Types**: Support for JSON, form, multipart, and no-body requests
- **Type Safety**: Compile-time validation of request structure

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
reqwest-builder = { version = "0.2.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### Basic Example

```rust
use reqwest_builder::{IntoReqwestBuilder, RequestBody};
use reqwest_builder_derive::IntoReqwestBuilder;
use serde::Serialize;

#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/users/{id}")]
struct GetUserRequest {
    #[path_param]
    id: u64,

    #[query]
    include_posts: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = url::Url::parse("https://api.example.com")?;

    let request = GetUserRequest {
        id: 123,
        include_posts: Some(true),
    };

    let builder = request.into_reqwest_builder(&client, &base_url)?;
    let response = builder.send().await?;

    println!("Status: {}", response.status());
    Ok(())
}
```

## Attributes Reference

### Container Attributes

These attributes are applied to the struct itself:

#### `#[request(method = "...")]` (Required)

Specifies the HTTP method for the request.

**Supported methods:**

- `GET`
- `POST`
- `PUT`
- `DELETE`
- `PATCH`
- `HEAD`
- `OPTIONS`

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/users")]
struct CreateUserRequest {
    // ...
}
```

#### `#[request(path = "...")]` (Required)

Specifies the endpoint path. Can include placeholders for path parameters.

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/users/{id}/posts/{post_id}")]
struct GetPostRequest {
    #[path_param]
    id: u64,
    #[path_param]
    post_id: u64,
}
```

#### `#[request(body = "...")]` (Optional)

Specifies how the request body should be encoded. Defaults to `"json"`.

**Supported body types:**

- `"json"` - JSON encoding (default)
- `"form"` - Form URL encoding
- `"multipart"` - Multipart form data
- `"none"` - No request body

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/login", body = "form")]
struct LoginRequest {
    username: String,
    password: String,
}
```

### Field Attributes

These attributes are applied to individual struct fields:

#### `#[path_param]`

Marks a field as a path parameter. The field's value will replace `{field_name}` in the path.

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/users/{user_id}/posts/{id}")]
struct GetPostRequest {
    #[path_param]
    user_id: u64,
    #[path_param]
    id: u64,
}
```

#### `#[query]` and `#[query(name = "...")]`

Marks a field as a query parameter.

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/posts")]
struct ListPostsRequest {
    #[query]
    page: Option<u32>,

    #[query(name = "per_page")]
    limit: Option<u32>,

    #[query]
    published: Option<bool>,
}
```

This generates a URL like: `/posts?page=1&per_page=10&published=true`

#### `#[header]` and `#[header(name = "...")]`

Marks a field as a request header.

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/api/data")]
struct ApiRequest {
    #[header(name = "Authorization")]
    auth_token: String,

    #[header(name = "Content-Type")]
    content_type: String,

    #[header]
    user_agent: String,  // Uses field name as header name

    // Body fields
    data: String,
}
```

#### `#[body]`

Explicitly marks a field to be included in the request body. This is the default behavior for unmarked fields, so it's usually not necessary.

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/users")]
struct CreateUserRequest {
    #[header(name = "Authorization")]
    auth_token: String,

    #[body]  // Explicit, but not necessary
    name: String,

    email: String,  // Implicitly included in body
}
```

## Advanced Examples

### Complex Request with All Features

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/users/{id}/posts", body = "json")]
struct CreatePostRequest {
    // Path parameter
    #[path_param]
    id: u64,

    // Query parameters
    #[query]
    draft: Option<bool>,

    #[query(name = "notify_followers")]
    should_notify: Option<bool>,

    // Headers
    #[header(name = "Authorization")]
    auth_token: String,

    #[header(name = "Content-Type")]
    content_type: String,

    #[header(name = "X-Client-Version")]
    client_version: String,

    // Request body fields
    title: String,
    content: String,
    tags: Vec<String>,
    metadata: serde_json::Value,
}
```

### Form-Encoded Request

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/auth/login", body = "form")]
struct LoginRequest {
    #[header(name = "User-Agent")]
    user_agent: String,

    username: String,
    password: String,
    remember_me: Option<bool>,
}
```

### GET Request with Query Parameters Only

```rust
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/search", body = "none")]
struct SearchRequest {
    #[query]
    q: String,

    #[query]
    page: Option<u32>,

    #[query(name = "sort_by")]
    sort: Option<String>,

    #[header(name = "Accept")]
    accept: String,
}
```

## Type Compatibility

The derive macro works with various Rust types:

- **Primitive types**: `String`, `u32`, `i64`, `bool`, etc.
- **Option types**: `Option<T>` for optional query parameters and headers
- **Collections**: `Vec<T>`, `HashMap<K, V>` (in body)
- **Custom types**: Any type that implements `Serialize` (for body fields)

## Error Handling

The generated implementation works with error handling approach provided by `reqwest-builder`:

```rust
// Explicit error handling
match request.into_reqwest_builder(&client, &base_url) {
    Ok(builder) => {
        let response = builder.send().await?;
        // Handle response
    }
    Err(e) => {
        eprintln!("Request building failed: {}", e);
    }
}
```

## Generated Code

The derive macro generates:

1. A companion `Headers` struct for typed header management
2. Implementation of all `IntoReqwestBuilder` trait methods:
   - `method()` - Returns the HTTP method
   - `endpoint()` - Builds the URL with path parameter substitution
   - `headers()` - Creates headers from annotated fields
   - `query_params()` - Builds query parameters from annotated fields
   - `body()` - Specifies the body encoding type

## Requirements

- Rust 2024 edition or later
- `serde` with `derive` feature for serialization
- `reqwest-builder` as the main crate

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
