use crate::player::{
    StdbOwnPlayerSessionV1, StdbOwnPlayerV1, StdbPubPlayerCardV1, stdb_own_player_session_v1, stdb_own_player_v1,
    stdb_pub_player_card_v1,
};
use spacetimedb::{Identity, ReducerContext, Timestamp};
use std::borrow::Borrow;
use stdb_common::prelude::{ResultExt, ServiceResult, Uuid, UuidExt, validate_str};

/// Repository trait for managing player session operations.
///
/// Handles session lifecycle including finding, signing in, and signing out players.
/// Sessions track online/offline state and link to player identities.
pub trait PlayerSessionRepository {
    /// Finds an existing player session by session ID.
    ///
    /// Returns `None` if no session exists for the given ID.
    fn find_session(&self, session_id: Identity) -> Option<StdbOwnPlayerSessionV1>;

    /// Signs in a player session, creating a new one if it doesn't exist.
    ///
    /// Sets the session to online and updates the player's sign-in timestamp.
    /// If this is a new session, also creates a new player record.
    ///
    /// # Errors
    /// Returns error if database operations fail.
    fn sign_in_session(&self, session_id: Identity) -> ServiceResult<StdbOwnPlayerSessionV1>;

    /// Signs out a player session by setting it to offline.
    ///
    /// Updates the player's last sign-out timestamp. No-op if session doesn't exist.
    ///
    /// # Errors
    /// Returns error if database operations fail.
    fn sign_out_session(&self, session_id: Identity) -> ServiceResult<()>;
}

/// Repository trait for managing player data operations.
///
/// Handles player records, display names, avatars, and public player cards.
/// Supports both private player data and public player card information.
pub trait PlayerRepository {
    /// Finds a player by their unique player ID.
    ///
    /// Returns `None` if no player exists with the given ID.
    fn find_player(&self, player_id: &Uuid) -> Option<StdbOwnPlayerV1>;

    /// Finds a player by their display name.
    ///
    /// Display names must be unique across all players.
    /// Returns `None` if no player has the given display name.
    fn find_player_by_display_name(&self, display_name: impl Borrow<String>) -> Option<StdbOwnPlayerV1>;

    /// Finds a public player card by player ID.
    ///
    /// Player cards contain publicly visible information like display name and avatar.
    /// Returns `None` if no card exists for the given player ID.
    fn find_player_card(&self, player_id: &Uuid) -> Option<StdbPubPlayerCardV1>;

    /// Creates a new player with the given ID, or returns existing player.
    ///
    /// Generates a unique random display name and creates both private player
    /// record and public player card. Idempotent - returns existing player if found.
    ///
    /// # Errors
    /// Returns error if database operations fail or validation fails.
    fn insert_player(&self, player_id: Uuid) -> ServiceResult<StdbOwnPlayerV1>;

    /// Creates or updates a player with custom display name and avatar.
    ///
    /// Validates display name (8-64 chars) and avatar (8-64 chars).
    /// Updates both private player record and public player card.
    ///
    /// # Errors
    /// Returns error if validation fails or database operations fail.
    fn upsert_player_card(
        &self,
        player_id: Uuid,
        display_name: impl Into<String>,
        avatar: impl Into<String>,
    ) -> ServiceResult<StdbOwnPlayerV1>;
}

impl PlayerSessionRepository for ReducerContext {
    fn find_session(&self, session_id: Identity) -> Option<StdbOwnPlayerSessionV1> {
        self.db.stdb_own_player_session_v1().session_id().find(session_id)
    }

    fn sign_in_session(&self, session_id: Identity) -> ServiceResult<StdbOwnPlayerSessionV1> {
        let mut session = self.find_session(session_id).unwrap_or_else(|| StdbOwnPlayerSessionV1 {
            session_id,
            player_id: self.new_uuid_v7(),
            is_online: true,
        });
        session.is_online = true;

        let session = self
            .db
            .stdb_own_player_session_v1()
            .session_id()
            .try_insert_or_update(session)
            .map_bad_request_ctx("failed to sign in player session")?;

        match self.find_player(&session.player_id) {
            Some(mut player) => {
                player.signed_in_at = self.timestamp;
                self.db.stdb_own_player_v1().player_id().update(player);
            },
            None => {
                self.insert_player(session.player_id.clone())?;
            },
        }

        Ok(session)
    }

    fn sign_out_session(&self, session_id: Identity) -> ServiceResult<()> {
        let Some(mut session) = self.find_session(session_id) else {
            return Ok(());
        };

        session.is_online = false;
        let session = self
            .db
            .stdb_own_player_session_v1()
            .session_id()
            .try_insert_or_update(session)
            .map_bad_request_ctx("failed to sign out player session")?;

        if let Some(mut player) = self.find_player(&session.player_id) {
            player.last_signed_out_at = self.timestamp;
            self.db.stdb_own_player_v1().player_id().update(player);
        }

        Ok(())
    }
}

impl PlayerRepository for ReducerContext {
    fn find_player(&self, player_id: &Uuid) -> Option<StdbOwnPlayerV1> {
        self.db.stdb_own_player_v1().player_id().find(player_id)
    }

    fn find_player_by_display_name(&self, display_name: impl Borrow<String>) -> Option<StdbOwnPlayerV1> {
        self.db.stdb_own_player_v1().display_name().find(display_name)
    }

    fn find_player_card(&self, player_id: &Uuid) -> Option<StdbPubPlayerCardV1> {
        self.db.stdb_pub_player_card_v1().player_id().find(player_id)
    }

    fn insert_player(&self, player_id: Uuid) -> ServiceResult<StdbOwnPlayerV1> {
        match self.find_player(&player_id) {
            Some(player) => Ok(player),
            None => {
                let display_name = build_unique_display_name(self);
                self.upsert_player_card(player_id, display_name, "default_avatar")
            },
        }
    }

    fn upsert_player_card(
        &self,
        player_id: Uuid,
        display_name: impl Into<String>,
        avatar: impl Into<String>,
    ) -> ServiceResult<StdbOwnPlayerV1> {
        let display_name = display_name.into();
        let avatar = avatar.into();

        validate_str("display_name", &display_name, 8, 64)?;
        validate_str("avatar", &avatar, 8, 64)?;

        let player = match self.find_player(&player_id) {
            Some(mut player) => {
                player.display_name = display_name;
                player.avatar = avatar;
                player
            },
            None => StdbOwnPlayerV1 {
                player_id,
                display_name,
                avatar,
                created_at: self.timestamp,
                signed_in_at: self.timestamp,
                last_signed_out_at: Timestamp::UNIX_EPOCH,
            },
        };

        let player = self
            .db
            .stdb_own_player_v1()
            .player_id()
            .try_insert_or_update(player)
            .map_conflict_ctx("failed to insert or update player")?;

        let card = StdbPubPlayerCardV1::from(player.clone());
        self.db
            .stdb_pub_player_card_v1()
            .player_id()
            .try_insert_or_update(card)
            .map_conflict_ctx("failed to insert or update player card")?;

        Ok(player)
    }
}

fn build_unique_display_name(ctx: &ReducerContext) -> String {
    for _ in 0..12 {
        let display_name = build_random_display_name(ctx);
        let existing = ctx.find_player_by_display_name(&display_name);
        if existing.is_none() {
            return display_name;
        }
    }

    // Hack... Horrible display name, but we can't find any cool ones.
    ctx.new_uuid_v4()
}

fn build_random_display_name(ctx: &ReducerContext) -> String {
    let color_index = ctx.random::<u8>() as usize % COLORS.len();
    let color = COLORS[color_index];

    let adjective_index = ctx.random::<u8>() as usize % ADJECTIVES.len();
    let adjective = ADJECTIVES[adjective_index];

    // Randomly choose between creatures and plants
    let use_creature = ctx.random::<bool>();
    let noun = if use_creature {
        let creature_index = ctx.random::<u8>() as usize % CREATURES.len();
        CREATURES[creature_index]
    } else {
        let plant_index = ctx.random::<u8>() as usize % PLANTS.len();
        PLANTS[plant_index]
    };

    format!("{} {} {}", color, adjective, noun)
}

const COLORS: &[&str] = &[
    "Red",
    "Blue",
    "Green",
    "Yellow",
    "Orange",
    "Purple",
    "Pink",
    "Brown",
    "Black",
    "White",
    "Gray",
    "Teal",
    "Cyan",
    "Magenta",
    "Maroon",
    "Navy",
    "Olive",
    "Lime",
    "Coral",
    "Turquoise",
    "Gold",
    "Silver",
    "Bronze",
    "Ivory",
    "Beige",
    "Lavender",
    "Mint",
    "Peach",
    "Amber",
    "Crimson",
    "Azure",
    "Emerald",
    "Violet",
    "Indigo",
    "Ruby",
    "Sapphire",
    "Onyx",
    "Pearl",
    "Jade",
    "Topaz",
    "Garnet",
    "Obsidian",
    "Copper",
    "Steel",
    "Platinum",
    "Charcoal",
    "Sand",
    "Rose",
];

const ADJECTIVES: &[&str] = &[
    "Swift",
    "Brave",
    "Mighty",
    "Silent",
    "Golden",
    "Ancient",
    "Fierce",
    "Noble",
    "Mystic",
    "Crystal",
    "Shadow",
    "Bright",
    "Wild",
    "Gentle",
    "Storm",
    "Fire",
    "Frost",
    "Thunder",
    "Lightning",
    "Celestial",
    "Radiant",
    "Crimson",
    "Azure",
    "Emerald",
    "Silver",
    "Cosmic",
    "Eternal",
    "Primal",
    "Savage",
    "Serene",
    "Blazing",
    "Frozen",
    "Winged",
    "Iron",
    "Steel",
    "Diamond",
    "Ruby",
    "Sapphire",
];

const CREATURES: &[&str] = &[
    "Wolf",
    "Eagle",
    "Tiger",
    "Dragon",
    "Phoenix",
    "Bear",
    "Hawk",
    "Lion",
    "Panther",
    "Falcon",
    "Raven",
    "Stag",
    "Fox",
    "Lynx",
    "Owl",
    "Shark",
    "Viper",
    "Cobra",
    "Stallion",
    "Leopard",
    "Jaguar",
    "Condor",
    "Bison",
    "Rhino",
    "Elk",
    "Moose",
    "Badger",
    "Wolverine",
    "Cougar",
    "Cheetah",
    "Orca",
    "Dolphin",
    "Whale",
    "Kraken",
    "Turtle",
    "Tortoise",
    "Gecko",
    "Iguana",
    "Salamander",
    "Newt",
    "Toad",
    "Frog",
    "Heron",
    "Crane",
    "Pelican",
    "Albatross",
    "Petrel",
    "Penguin",
    "Seal",
    "Walrus",
];

const PLANTS: &[&str] = &[
    "Oak",
    "Pine",
    "Redwood",
    "Cedar",
    "Birch",
    "Maple",
    "Willow",
    "Ash",
    "Elder",
    "Yew",
    "Cypress",
    "Sequoia",
    "Magnolia",
    "Cherry",
    "Bamboo",
    "Lotus",
    "Rose",
    "Orchid",
    "Lily",
    "Iris",
    "Tulip",
    "Sunflower",
    "Lavender",
    "Jasmine",
    "Sage",
    "Thyme",
    "Basil",
    "Mint",
    "Rosemary",
    "Fern",
    "Moss",
    "Ivy",
    "Vine",
    "Heather",
    "Thistle",
    "Clover",
    "Daisy",
];
