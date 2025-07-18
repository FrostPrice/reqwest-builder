use crate::{
    errors::ReqwestBuilderError,
    serialization::{construct_url, serialize_to_form_params, serialize_to_header_map},
    types::{QueryParams, RequestBody},
};
use serde::Serialize;
use url::Url;

/// Trait for converting request structures into reqwest builders
///
/// This trait provides a standardized way to convert request types into
/// `reqwest_middleware::RequestBuilder` instances with proper configuration.
pub trait IntoReqwestBuilder
where
    Self: Sized + Serialize,
{
    /// Associated type for request headers
    type Headers: Serialize + Clone;

    /// HTTP method for the request
    fn method(&self) -> http::Method;

    /// Endpoint path for the request
    fn endpoint(&self) -> String;

    /// Optional headers for the request
    fn headers(&self) -> Option<Self::Headers> {
        None
    }

    /// Request body type
    fn body(&self) -> RequestBody {
        RequestBody::Json
    }

    /// Optional query parameters
    fn query_params(&self) -> Option<QueryParams> {
        None
    }

    /// Create multipart form - override this for file uploads
    fn create_multipart_form(&self) -> Option<reqwest::multipart::Form> {
        None
    }

    /// Convert the request into a reqwest builder with proper error handling
    ///
    /// This is the preferred method for new code as it provides proper error handling.
    fn into_reqwest_builder(
        self,
        client: &reqwest_middleware::ClientWithMiddleware,
        base_url: &Url,
    ) -> std::result::Result<reqwest_middleware::RequestBuilder, ReqwestBuilderError> {
        // Construct URL with error handling
        let url = construct_url(base_url, &self.endpoint());
        let mut builder = client.request(self.method(), &url);

        // Add query parameters if present
        if let Some(params) = self.query_params() {
            builder = builder.query(&params);
        }

        // Handle request body with error handling
        builder = self.add_body_to_builder(builder)?;

        // Add headers with error handling
        if let Some(headers) = self.headers() {
            let header_map = serialize_to_header_map(&headers)?;
            builder = builder.headers(header_map);
        }

        Ok(builder)
    }

    /// Add body to the request builder with proper error handling
    fn add_body_to_builder(
        &self,
        mut builder: reqwest_middleware::RequestBuilder,
    ) -> std::result::Result<reqwest_middleware::RequestBuilder, ReqwestBuilderError> {
        match self.body() {
            RequestBody::Json => {
                let json_str = serde_json::to_string(self).map_err(ReqwestBuilderError::from)?;
                if json_str != "{}" {
                    builder = builder.json(self);
                }
            }
            RequestBody::Form => {
                let params = serialize_to_form_params(self)?;
                builder = builder.form(&params);
            }
            RequestBody::Multipart => {
                if let Some(form) = self.create_multipart_form() {
                    builder = builder.multipart(form);
                }
            }
            RequestBody::None => {
                // No body to add
            }
        }
        Ok(builder)
    }
}

// Helper function for the derive macro to handle query parameters
// This works with both Option and non-Option types
pub fn query_param_helper<T>(
    value: &T,
    param_name: &str,
    params: &mut std::collections::HashMap<String, String>,
) where
    T: QueryParamValue,
{
    value.add_to_params(param_name, params);
}

// Trait to handle different types of query parameter values
pub trait QueryParamValue {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    );
}

// Implementation for Option types
impl<T: std::fmt::Display> QueryParamValue for Option<T> {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        if let Some(value) = self {
            params.insert(param_name.to_string(), value.to_string());
        }
    }
}

// Implementations for common non-Option types
/// TODO: We should use a better aproach to handle these types
impl QueryParamValue for String {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.clone());
    }
}

impl QueryParamValue for &str {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}

impl QueryParamValue for u32 {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}

impl QueryParamValue for u64 {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}

impl QueryParamValue for i32 {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}

impl QueryParamValue for i64 {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}

impl QueryParamValue for bool {
    fn add_to_params(
        &self,
        param_name: &str,
        params: &mut std::collections::HashMap<String, String>,
    ) {
        params.insert(param_name.to_string(), self.to_string());
    }
}
