# stdb-inventory

Inventory and item management system for SpacetimeDB-based games. This library provides tables and functionality to handle player inventories, item definitions, trading, and equipment management.

## Features

### Inventory Management
- **Player inventories**: Store and manage player-owned items
- **Item stacking**: Automatic stacking for stackable items
- **Equipment slots**: Manage equipped items and loadouts
- **Storage containers**: Shared storage like guild storage

### Database Tables
- **PlayerInventory**: Stores player-owned items and quantities
- **ItemDefinition**: Master catalog of all available items
- **EquippedItems**: Tracks currently equipped gear per player
- **ItemTransaction**: Logs item transfers, trades, and purchases

### Item Types
- **Consumables**: Potions, food, temporary buffs
- **Equipment**: Weapons, armor, accessories
- **Materials**: Crafting components and resources
- **Quest items**: Special items for quests and progression

Run tests with:
```bash
cd server
cargo +nightly fmt && cargo check --all && cargo test
```
