# stdb-http

HTTP request management utilities for SpacetimeDB-based game development. This library provides tables and functionality to enable backend servers to execute HTTP requests within the SpacetimeDB environment.

## Features

### HTTP Request Management
- **Request queuing**: Store HTTP requests in SpacetimeDB tables for processing
- **Response handling**: Capture and store HTTP responses with proper error handling
- **Request validation**: Validate HTTP request parameters and headers
- **Retry logic**: Built-in retry mechanisms for failed requests

### Database Tables
- **HttpRequest**: Stores outbound HTTP request details
- **HttpResponse**: Captures response data and status codes
- **HttpRequestQueue**: Manages request processing queue with priorities
- **HttpRetryPolicy**: Configures retry behavior for failed requests

### Request Types
- **GET requests**: Simple data retrieval operations
- **POST requests**: Data submission with JSON payloads
- **PUT/PATCH requests**: Resource updates and modifications
- **DELETE requests**: Resource removal operations

Run tests with:
```bash
cd server
cargo +nightly fmt && cargo check --all && cargo test
```
