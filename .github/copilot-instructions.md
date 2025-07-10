# Copilot Instructions

## Project Context
This is a SpacetimeDB library collection providing server-side utilities and future client integrations for game development.

## Code Style
- Use Rust best practices and idiomatic code
- Follow existing error handling patterns with `ServiceResult<T>`
- Prefer explicit types and meaningful variable names
- Use `#[must_use]` for validation functions
- Write comprehensive unit tests for all public functions

## Architecture Guidelines
- Keep validation logic in `stdb-common`
- Player-specific logic goes in `stdb-player`
- Use traits for extensibility (e.g., `UuidExt`)
- Leverage SpacetimeDB's `ReducerContext` for randomness and timestamps

## Error Handling
- Use `ValidationError` for input validation failures
- Map errors through `ErrorMapper` trait
- Provide descriptive error messages with field names

## Testing
- Test both success and failure cases
- Use deterministic inputs for reproducible tests
- Test version bits and variant bits for UUID generation

## Dependencies
- Minimize external dependencies
- Prefer standard library solutions
- Use `spacetimedb` framework features when available
