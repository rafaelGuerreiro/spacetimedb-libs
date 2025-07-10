# stdb-match

Matchmaking and game session management utilities for SpacetimeDB-based multiplayer games. This library provides tables and functionality to handle player matchmaking, lobby creation, and game session lifecycle management.

## Features

### Matchmaking System
- **Player queuing**: Queue players for matchmaking with skill-based criteria
- **Lobby management**: Create and manage game lobbies with configurable settings
- **Session tracking**: Monitor active game sessions and player connections
- **Skill-based matching**: Match players based on rating, level, or custom criteria

### Database Tables
- **MatchQueue**: Stores players waiting for matches with preferences
- **GameLobby**: Manages lobby state, settings, and player lists
- **GameSession**: Tracks active game sessions and their status
- **PlayerRating**: Maintains player skill ratings and statistics

### Match Types
- **Ranked matches**: Competitive games with rating adjustments
- **Casual matches**: Quick play without rating impact
- **Custom lobbies**: Player-created rooms with custom rules
- **Tournament brackets**: Structured competitive play

Run tests with:
```bash
cd server
cargo +nightly fmt && cargo check --all && cargo test
```
