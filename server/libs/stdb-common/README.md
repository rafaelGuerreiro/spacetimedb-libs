# stdb-common

Core utilities and common functionality for SpacetimeDB-based game development. This library provides essential building blocks including validation, error handling, and UUID generation.

## Features

### Validation
- **String validation**: Length constraints with descriptive error messages
- **Numeric validation**: Range validation for all unsigned integer types (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`)
- **Reducer context requirements**: Access control validation for private operations

### UUID Generation
- **UUID v4**: Cryptographically random UUIDs using SpacetimeDB's random number generator
- **UUID v7**: Timestamp-based UUIDs for sortable identifiers
- **String representation**: UUIDs are represented as strings for SpacetimeDB compatibility

### Error Handling
- **Structured errors**: `ValidationError` enum with specific error types
- **Error mapping**: `ErrorMapper` trait for consistent error transformation
- **Service results**: `ServiceResult<T>` type alias for standardized error handling

## Usage

### Validation

```rust
use stdb_common::{validate_str, validate_u32, ServiceResult};

fn create_player(name: &str, level: u32) -> ServiceResult<()> {
    validate_str("name", name, 3, 20)?;
    validate_u32("level", level, 1, 100)?;
    Ok(())
}
```

### UUID Generation

```rust
use stdb_common::{UuidExt, Uuid};
use spacetimedb::ReducerContext;

fn create_unique_id(ctx: &ReducerContext) -> Uuid {
    // Random UUID v4
    let random_id = ctx.new_uuid_v4();

    // Timestamp-based UUID v7
    let timestamp_id = ctx.new_uuid_v7();

    random_id
}
```

### Reducer Context Requirements

```rust
use stdb_common::ReducerContextRequirements;
use spacetimedb::ReducerContext;

fn admin_only_operation(ctx: &ReducerContext) -> ServiceResult<()> {
    ctx.require_private_access()?;
    // Perform admin operation
    Ok(())
}
```

## Error Types

### ValidationError
- `RequiredField`: Field is required but empty
- `FieldTooSmall`: Value is below minimum threshold
- `FieldTooLarge`: Value exceeds maximum threshold

## API Reference

### Functions
- `validate_str(name, value, min_length, max_length)`: Validates string length
- `validate_u8/u16/u32/u64/u128/usize(name, value, min_value, max_value)`: Validates numeric ranges

### Traits
- `UuidExt`: Extends `ReducerContext` with UUID generation methods
- `ReducerContextRequirements`: Provides access control validation
- `ErrorMapper`: Maps errors to `ServiceError`

### Types
- `Uuid`: String representation of UUID
- `ServiceResult<T>`: Result type with `ServiceError`
- `ValidationError`: Validation-specific error enum

## Testing

The library includes comprehensive unit tests covering:
- UUID version and variant bit validation
- String validation edge cases
- Numeric validation boundaries
- Error message formatting

Run tests with:
```bash
cd server
cargo +nightly fmt && cargo check --all && cargo test
```
