//! # Reqwest Builder
//!
//! A builder for reqwest requests with support for custom headers, query parameters,
//! and body content, featuring comprehensive error handling.
//!
//! ## Features
//!
//! - **Builder Pattern**: Trait-based approach for converting request structures into reqwest builders
//! - **Multiple Body Types**: Support for JSON, form-encoded, multipart, and no-body requests
//! - **Error Handling**: Comprehensive error handling with detailed error messages
//! - **File Uploads**: Built-in support for file uploads with MIME type detection
//! - **Header Management**: Safe header serialization with proper error reporting
//!
//! ## Quick Start
//!
//! ```rust
//! use reqwest_builder::{IntoReqwestBuilder, RequestBody};
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct CreateUserRequest {
//!     name: String,
//!     email: String,
//! }
//!
//! impl IntoReqwestBuilder for CreateUserRequest {
//!     type Headers = ();
//!
//!     fn method(&self) -> http::Method {
//!         http::Method::POST
//!     }
//!
//!     fn endpoint(&self) -> String {
//!         "/users".to_string()
//!     }
//! }
//! ```

// Core modules
pub mod errors;
pub mod file_upload;
pub mod serialization;
pub mod trait_impl;
pub mod types;

// Feature-gated modules
#[cfg(feature = "derive")]
pub use reqwest_builder_derive::*;

// Re-exports for convenience
pub use errors::ReqwestBuilderError;
pub use file_upload::FileUpload;
pub use trait_impl::{IntoReqwestBuilder, QueryParamValue, query_param_helper};
pub use types::{QueryParams, RequestBody};

// Re-export serialization functions for advanced users
pub use serialization::{construct_url, serialize_to_form_params, serialize_to_header_map};
