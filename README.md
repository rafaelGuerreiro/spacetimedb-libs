# spacetimedb-libs

A collection of built-in functionalities and integrations designed to accelerate game development with SpacetimeDB. This repository provides both server-side libraries and client-side integrations to help developers quickly add backend capabilities to their games.

## Project Structure

### `/server`
Contains Rust libraries that provide common backend functionalities for SpacetimeDB-based games:

- **`stdb-common`**: Core utilities including validation, error handling, and UUID generation
- **`stdb-player`**: Player management logic with UUID-based public identifiers

### `/godot` (Coming Soon)
Will contain Godot-specific implementations and integrations for seamless client-server communication with SpacetimeDB.

## Getting Started

### Server Libraries

To use the server libraries in your SpacetimeDB project, add them to your `Cargo.toml`:

```toml
[dependencies]
stdb-common = { version = "0.1.0" }
stdb-player = { version = "0.1.0" }
```

### Example Usage

```rust
use stdb_common::{validate_str, UuidExt};
use stdb_player::Player;
use spacetimedb::ReducerContext;

// Generate UUIDs
let player_id = ctx.new_uuid_v4();

// Validate input
validate_str("display_name", &display_name, 3, 20)?;

// Create player
let player = Player::new(display_name, player_id);
```

## Features

- **Validation**: Comprehensive input validation for strings and numeric types
- **UUID Generation**: Support for UUID v4 (random) and UUID v7 (timestamp-based)
- **Error Handling**: Structured error types with proper error mapping
- **Player Management**: Basic player logic with unique identifiers

## Development

This project uses Rust with Cargo workspaces. To build all libraries:

```bash
cd server
cargo +nightly fmt && cargo check --all && cargo build
```

To run tests:

```bash
cd server
cargo +nightly fmt && cargo check --all && cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the terms of the MIT license. See the LICENSE file for details.
