/// Custom error types for the reqwest-builder library
#[derive(Debug, Clone, PartialEq)]
pub enum ReqwestBuilderError {
    /// Error serializing data to JSON
    SerializationError(String),
    /// Error with header name or value
    HeaderError {
        key: String,
        value: String,
        source: String,
    },
    /// Error constructing URL
    UrlError(String),
    /// File I/O error
    IoError(String),
    /// Invalid request configuration
    InvalidRequest(String),
}

impl std::fmt::Display for ReqwestBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReqwestBuilderError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            ReqwestBuilderError::HeaderError { key, value, source } => {
                write!(f, "Header error for '{}': '{}' - {}", key, value, source)
            }
            ReqwestBuilderError::UrlError(msg) => write!(f, "URL error: {}", msg),
            ReqwestBuilderError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ReqwestBuilderError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
        }
    }
}

impl std::error::Error for ReqwestBuilderError {}

impl From<std::io::Error> for ReqwestBuilderError {
    fn from(err: std::io::Error) -> Self {
        ReqwestBuilderError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for ReqwestBuilderError {
    fn from(err: serde_json::Error) -> Self {
        ReqwestBuilderError::SerializationError(err.to_string())
    }
}
