use crate::errors::ReqwestBuilderError;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::result::Result<Self, ReqwestBuilderError> {
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
