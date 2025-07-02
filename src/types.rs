use std::collections::HashMap;

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
