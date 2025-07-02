use reqwest_builder::IntoReqwestBuilder;
use serde::Serialize;
use url::Url;

// A simple test case for the derive macro without path parameters
#[derive(Serialize, IntoReqwestBuilder)]
#[request(method = "GET", path = "/users")]
struct SimpleGetRequest {
    #[query]
    limit: Option<u32>,

    #[header(name = "Authorization")]
    auth_token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Derive Macro Test ===");

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    let base_url = Url::parse("https://api.example.com")?;

    let request = SimpleGetRequest {
        limit: Some(10),
        auth_token: "Bearer test123".to_string(),
    };

    // Test the basic functionality
    println!("Method: {:?}", request.method());
    println!("Endpoint: {}", request.endpoint());

    if let Some(_headers) = request.headers() {
        println!("Has headers: yes");
    } else {
        println!("Has headers: no");
    }

    if let Some(params) = request.query_params() {
        println!("Query params: {:?}", params);
    } else {
        println!("Query params: none");
    }

    println!("Body type: {:?}", request.body());

    // Try to build the request
    match request.into_reqwest_builder(&client, &base_url) {
        Ok(_builder) => {
            println!("Request builder created successfully!");
        }
        Err(e) => {
            println!("Error creating request builder: {}", e);
        }
    }

    Ok(())
}
