use reqwest_builder::IntoReqwestBuilder;
use serde::Serialize;
use url::Url;

// Example 1: Simple GET request with path parameter
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/users/{id}")]
struct GetUserRequest {
    #[path_param]
    id: u64,

    #[query]
    include_posts: Option<bool>,

    #[query(name = "format")]
    response_format: Option<String>,
}

// Example 2: POST request with headers and body
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/users/{user_id}/posts", body = "json")]
struct CreatePostRequest {
    #[path_param]
    user_id: u64,

    #[header(name = "Authorization")]
    auth_token: String,

    #[header(name = "Content-Type")]
    content_type: String,

    #[query]
    draft: Option<bool>,

    // These fields go into the request body
    title: String,
    content: String,
    tags: Vec<String>,
}

// Example 3: Form-encoded request
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "POST", path = "/login", body = "form")]
struct LoginRequest {
    #[header(name = "User-Agent")]
    user_agent: String,

    username: String,
    password: String,
}

// Example 4: DELETE request with no body
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "DELETE", path = "/posts/{id}", body = "none")]
struct DeletePostRequest {
    #[path_param]
    id: u64,

    #[header(name = "Authorization")]
    auth_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Reqwest Builder Derive Macro Demo ===\n");

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = Url::parse("https://api.example.com")?;

    // Example 1: GET user with query parameters
    println!("1. GET User Request:");
    let get_user = GetUserRequest {
        id: 123,
        include_posts: Some(true),
        response_format: Some("json".to_string()),
    };

    let builder = get_user.into_reqwest_builder(&client, &base_url)?;
    println!("   URL: {}", builder.try_clone().unwrap().build()?.url());
    println!(
        "   Method: {:?}",
        builder.try_clone().unwrap().build()?.method()
    );

    // Example 2: POST with headers and body
    println!("\n2. Create Post Request:");
    let create_post = CreatePostRequest {
        user_id: 456,
        auth_token: "Bearer abc123".to_string(),
        content_type: "application/json".to_string(),
        draft: Some(false),
        title: "My New Post".to_string(),
        content: "This is the content of my post.".to_string(),
        tags: vec!["rust".to_string(), "programming".to_string()],
    };

    let builder = create_post.into_reqwest_builder(&client, &base_url)?;
    println!("   URL: {}", builder.try_clone().unwrap().build()?.url());
    println!(
        "   Method: {:?}",
        builder.try_clone().unwrap().build()?.method()
    );
    println!(
        "   Headers: {:?}",
        builder.try_clone().unwrap().build()?.headers()
    );

    // Example 3: Form request
    println!("\n3. Login Request (Form):");
    let login = LoginRequest {
        user_agent: "MyApp/1.0".to_string(),
        username: "john_doe".to_string(),
        password: "secret123".to_string(),
    };

    let builder = login.into_reqwest_builder(&client, &base_url)?;
    println!("   URL: {}", builder.try_clone().unwrap().build()?.url());
    println!(
        "   Method: {:?}",
        builder.try_clone().unwrap().build()?.method()
    );

    // Example 4: DELETE request
    println!("\n4. Delete Post Request:");
    let delete_post = DeletePostRequest {
        id: 789,
        auth_token: "Bearer xyz789".to_string(),
    };

    let builder = delete_post.into_reqwest_builder(&client, &base_url)?;
    println!("   URL: {}", builder.try_clone().unwrap().build()?.url());
    println!(
        "   Method: {:?}",
        builder.try_clone().unwrap().build()?.method()
    );

    println!("\n=== Benefits of the Derive Macro ===");
    println!("No manual trait implementation needed");
    println!("Clear, declarative attribute syntax");
    println!("Automatic path parameter substitution");
    println!("Type-safe header and query parameter handling");
    println!("Supports all request body types");
    println!("Reduces boilerplate code significantly");

    Ok(())
}
