# Module Organization

This document explains how the reqwest-builder library is organized into modules.

## Module Structure

```txt
src/
├── lib.rs              # Main library file with re-exports and documentation
├── errors.rs           # Error types and implementations
├── types.rs            # Core types and enums (RequestBody, QueryParams)
├── trait_impl.rs       # Main IntoReqwestBuilder trait and implementation
├── serialization.rs    # Serialization helper functions
├── file_upload.rs      # File upload functionality
├── tests.rs            # All tests consolidated
└── derive/             # Procedural macro support (feature-gated)
```

## Module Responsibilities

### `lib.rs` - Main Library Interface

- Library documentation
- Module declarations
- Public re-exports
- Feature gates

### `errors.rs` - Error Handling

- `ReqwestBuilderError` enum
- Error trait implementations
- Conversion implementations (`From` traits)

### `types.rs` - Core Types

- `RequestBody` enum
- `QueryParams` type alias
- Other common types

### `trait_impl.rs` - Main Trait

- `IntoReqwestBuilder` trait definition
- Default trait implementations
- Request building logic
- Both safe and with error-handling method

### `serialization.rs` - Serialization Utilities

- Form parameter serialization (safe and with error-handling)
- Header map serialization (safe and with error-handling)
- URL construction utilities
- All JSON/serde-related helper functions

### `file_upload.rs` - File Operations

- `FileUpload` struct definition
- File reading and MIME type detection
- File-related error handling

### `tests.rs` - Test Suite

- All unit tests consolidated
- Tests for each module's functionality
- Integration tests

## Benefits of This Structure

### 1. **Separation of Concerns**

Each module has a clear, single responsibility, making the code easier to understand and maintain.

### 2. **Better Testability**

Individual components can be tested in isolation, and tests are organized in a dedicated module.

### 3. **Improved Maintainability**

Changes to one aspect (e.g., error handling) don't require touching unrelated code.

### 4. **Cleaner Public API**

The main `lib.rs` provides a clean interface with well-organized re-exports.

### 5. **Reduced Coupling**

Modules depend only on what they need, reducing tight coupling between components.

### 6. **Easier Navigation**

Developers can quickly find and focus on the specific functionality they need to work with.

## Import Patterns

### For End Users

```rust
use reqwest_builder::{IntoReqwestBuilder, RequestBody, ReqwestBuilderError, FileUpload};
```

### For Advanced Users (accessing utilities)

```rust
use reqwest_builder::{
    IntoReqwestBuilder,
    serialize_to_header_map,
    construct_url
};
```

### Internal Module Usage

```rust
use crate::{
    errors::ReqwestBuilderError,
    types::RequestBody,
    serialization::serialize_to_header_map,
};
```

## Future Extensibility

This modular structure makes it easy to:

- Add new request body types to `types.rs`
- Add new error variants to `errors.rs`
- Add new serialization methods to `serialization.rs`
- Add new file handling features to `file_upload.rs`
- Add procedural macros to the `derive/` module
