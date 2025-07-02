use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, parse_macro_input};

/// Derive macro for IntoReqwestBuilder
///
/// This macro allows you to define HTTP request properties directly on struct fields:
///
/// # Attributes
///
/// ## Container attributes (on the struct):
/// - `#[request(method = "GET|POST|PUT|DELETE|PATCH")]` - HTTP method (required)
/// - `#[request(path = "/endpoint")]` - Base endpoint path (required)
/// - `#[request(body = "json|form|multipart|none")]` - Body type (optional, defaults to "json")
///
/// ## Field attributes:
/// - `#[path_param]` - Include this field in the URL path (replaces `{field_name}` in path)
/// - `#[query]` - Include this field as a query parameter
/// - `#[query(name = "param_name")]` - Include as query parameter with custom name
/// - `#[header]` - Include this field as a header
/// - `#[header(name = "header_name")]` - Include as header with custom name
/// - `#[body]` - Include this field in the request body (default for unmarked fields)
///
/// # Example
///
/// ```rust
/// use reqwest_builder::{IntoReqwestBuilder, RequestBody};
/// use serde::Serialize;
///
/// #[derive(Serialize, IntoReqwestBuilder)]
/// #[request(method = "POST", path = "/users/{id}/posts")]
/// struct CreatePostRequest {
///     #[path_param]
///     id: u64,
///     
///     #[query]
///     draft: Option<bool>,
///     
///     #[header(name = "Authorization")]
///     auth_token: String,
///     
///     #[header(name = "Content-Type")]
///     content_type: String,
///     
///     // These fields go into the request body
///     title: String,
///     content: String,
/// }
/// ```
#[proc_macro_derive(
    IntoReqwestBuilder,
    attributes(request, path_param, query, header, body)
)]
pub fn derive_into_reqwest_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match impl_into_reqwest_builder(&input) {
        Ok(output) => output.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn impl_into_reqwest_builder(input: &DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let name = &input.ident;

    // Parse container attributes
    let container_attrs = parse_container_attributes(&input.attrs)?;
    let method = container_attrs.method;
    let path = container_attrs.path;
    let body_type = container_attrs.body_type;

    // Extract struct fields
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Only named fields are supported",
                ));
            }
        },
        _ => return Err(syn::Error::new_spanned(input, "Only structs are supported")),
    };

    // Analyze fields for different purposes
    let mut path_fields = Vec::new();
    let mut query_fields = Vec::new();
    let mut header_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_attrs = parse_field_attributes(&field.attrs)?;

        match field_attrs.field_type {
            FieldType::Path => {
                path_fields.push(field_name);
            }
            FieldType::Query { name } => {
                let param_name = name.unwrap_or_else(|| field_name.to_string());
                query_fields.push((field_name, param_name));
            }
            FieldType::Header { name } => {
                let header_name = name.unwrap_or_else(|| field_name.to_string());
                header_fields.push((field_name, header_name));
            }
            FieldType::Body => {
                // Body fields are handled automatically by serde serialization
                // We don't need to do anything special for them
            }
        }
    }

    // Generate the endpoint method with path substitution
    let endpoint_impl = generate_endpoint_impl(&path, &path_fields);

    // Generate query params method
    let query_params_impl = generate_query_params_impl(&query_fields);

    // Generate headers method and Headers type
    let (headers_type, headers_impl, headers_struct_name) =
        generate_headers_impl(name, &header_fields);

    // Generate the method implementation
    let method_impl = quote! {
        fn method(&self) -> http::Method {
            #method
        }
    };

    // Generate body type implementation
    let body_impl = quote! {
        fn body(&self) -> ::reqwest_builder::RequestBody {
            #body_type
        }
    };

    Ok(quote! {
        #headers_type

        impl ::reqwest_builder::IntoReqwestBuilder for #name {
            type Headers = #headers_struct_name;

            #method_impl

            #endpoint_impl

            #headers_impl

            #query_params_impl

            #body_impl
        }
    })
}

#[derive(Debug)]
struct ContainerAttributes {
    method: proc_macro2::TokenStream,
    path: String,
    body_type: proc_macro2::TokenStream,
}

#[derive(Debug)]
struct FieldAttributes {
    field_type: FieldType,
}

#[derive(Debug)]
enum FieldType {
    Path,
    Query { name: Option<String> },
    Header { name: Option<String> },
    Body,
}

fn parse_container_attributes(attrs: &[syn::Attribute]) -> Result<ContainerAttributes, syn::Error> {
    let mut method = None;
    let mut path = None;
    let mut body_type = quote! { reqwest_builder::RequestBody::Json }; // Default to JSON

    for attr in attrs {
        if attr.path().is_ident("request") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("method") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        method = Some(match lit_str.value().as_str() {
                            "GET" => quote! { http::Method::GET },
                            "POST" => quote! { http::Method::POST },
                            "PUT" => quote! { http::Method::PUT },
                            "DELETE" => quote! { http::Method::DELETE },
                            "PATCH" => quote! { http::Method::PATCH },
                            "HEAD" => quote! { http::Method::HEAD },
                            "OPTIONS" => quote! { http::Method::OPTIONS },
                            other => {
                                return Err(
                                    meta.error(format!("Unsupported HTTP method: {}", other))
                                );
                            }
                        });
                    }
                } else if meta.path.is_ident("path") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        path = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("body") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        body_type = match lit_str.value().as_str() {
                            "json" => quote! { reqwest_builder::RequestBody::Json },
                            "form" => quote! { reqwest_builder::RequestBody::Form },
                            "multipart" => quote! { reqwest_builder::RequestBody::Multipart },
                            "none" => quote! { reqwest_builder::RequestBody::None },
                            other => {
                                return Err(meta.error(format!("Unsupported body type: {}", other)));
                            }
                        };
                    }
                }
                Ok(())
            })?;
        }
    }

    let method = method
        .ok_or_else(|| syn::Error::new_spanned(&attrs[0], "Missing required 'method' attribute"))?;
    let path = path
        .ok_or_else(|| syn::Error::new_spanned(&attrs[0], "Missing required 'path' attribute"))?;

    Ok(ContainerAttributes {
        method,
        path,
        body_type,
    })
}

fn parse_field_attributes(attrs: &[syn::Attribute]) -> Result<FieldAttributes, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("path_param") {
            return Ok(FieldAttributes {
                field_type: FieldType::Path,
            });
        } else if attr.path().is_ident("query") {
            let mut name = None;

            // Try to parse nested meta if the attribute has arguments
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        name = Some(lit_str.value());
                    }
                }
                Ok(())
            });

            return Ok(FieldAttributes {
                field_type: FieldType::Query { name },
            });
        } else if attr.path().is_ident("header") {
            let mut name = None;

            // Try to parse nested meta if the attribute has arguments
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        name = Some(lit_str.value());
                    }
                }
                Ok(())
            });

            return Ok(FieldAttributes {
                field_type: FieldType::Header { name },
            });
        } else if attr.path().is_ident("body") {
            return Ok(FieldAttributes {
                field_type: FieldType::Body,
            });
        }
    }

    // Default to body field if no attribute is specified
    Ok(FieldAttributes {
        field_type: FieldType::Body,
    })
}

fn generate_endpoint_impl(path: &str, path_fields: &[&syn::Ident]) -> proc_macro2::TokenStream {
    if path_fields.is_empty() {
        quote! {
            fn endpoint(&self) -> String {
                #path.to_string()
            }
        }
    } else {
        let mut endpoint_code = quote! {
            let mut endpoint = #path.to_string();
        };

        for field in path_fields {
            let field_name_str = field.to_string();
            let placeholder = format!("{{{}}}", field_name_str);

            endpoint_code.extend(quote! {
                endpoint = endpoint.replace(#placeholder, &self.#field.to_string());
            });
        }

        endpoint_code.extend(quote! {
            endpoint
        });

        quote! {
            fn endpoint(&self) -> String {
                #endpoint_code
            }
        }
    }
}

fn generate_query_params_impl(query_fields: &[(&syn::Ident, String)]) -> proc_macro2::TokenStream {
    if query_fields.is_empty() {
        quote! {
            fn query_params(&self) -> Option<std::collections::HashMap<String, String>> {
                None
            }
        }
    } else {
        let param_insertions: Vec<_> = query_fields
            .iter()
            .map(|(field, param_name)| {
                quote! {
                    // Handle query parameters - this works for both Option and non-Option types
                    let field_ref = &self.#field;
                    reqwest_builder::query_param_helper(field_ref, #param_name, &mut params);
                }
            })
            .collect();

        quote! {
            fn query_params(&self) -> Option<std::collections::HashMap<String, String>> {
                let mut params = std::collections::HashMap::new();
                #(#param_insertions)*
                if params.is_empty() {
                    None
                } else {
                    Some(params)
                }
            }
        }
    }
}

fn generate_headers_impl(
    struct_name: &syn::Ident,
    header_fields: &[(&syn::Ident, String)],
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let headers_struct_name = quote::format_ident!("{}Headers", struct_name);

    if header_fields.is_empty() {
        let headers_type = quote! {
            #[derive(serde::Serialize, Clone)]
            pub struct #headers_struct_name;
        };

        let headers_impl = quote! {
            fn headers(&self) -> Option<Self::Headers> {
                None
            }
        };

        return (headers_type, headers_impl, quote! { #headers_struct_name });
    }

    let header_struct_fields: Vec<_> = header_fields
        .iter()
        .map(|(field, header_name)| {
            let field_type = quote! { String };
            quote! {
                #[serde(rename = #header_name)]
                pub #field: #field_type
            }
        })
        .collect();

    let headers_type = quote! {
        #[derive(serde::Serialize, Clone)]
        pub struct #headers_struct_name {
            #(#header_struct_fields),*
        }
    };

    let header_assignments: Vec<_> = header_fields
        .iter()
        .map(|(field, _)| {
            quote! {
                #field: self.#field.to_string()
            }
        })
        .collect();

    let headers_impl = quote! {
        fn headers(&self) -> Option<Self::Headers> {
            Some(#headers_struct_name {
                #(#header_assignments),*
            })
        }
    };

    (headers_type, headers_impl, quote! { #headers_struct_name })
}
