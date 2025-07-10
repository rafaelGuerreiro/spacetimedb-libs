use crate::{
    player::repository::{PlayerRepository, PlayerSessionRepository},
    prelude::PlayerExt,
};
use spacetimedb::{Filter, Identity, ReducerContext, Timestamp, client_visibility_filter, reducer, table};
use stdb_common::prelude::{ServiceResult, Uuid};

pub mod repository;

pub(crate) fn stdb_init(_ctx: &ReducerContext) -> ServiceResult<()> {
    Ok(())
}

pub(crate) fn stdb_identity_connected(ctx: &ReducerContext) -> ServiceResult<()> {
    ctx.sign_in_session(ctx.sender)?;
    Ok(())
}

pub(crate) fn stdb_identity_disconnected(ctx: &ReducerContext) {
    let _ = ctx.sign_out_session(ctx.sender);
}

#[client_visibility_filter]
const STDB_OWN_PLAYER_SESSION_V1_FILTER: Filter = Filter::Sql(
    r#"
    select s.*
    from stdb_own_player_session_v1 s
    where s.session_id = :sender
"#,
);

#[client_visibility_filter]
const STDB_OWN_PLAYER_V1_FILTER: Filter = Filter::Sql(
    r#"
    select s.*
    from stdb_own_player_v1 s
    where s.session_id = :sender
"#,
);

/// Session mapping table - tracks active sessions for player authentication.
///
/// Different devices/clients for the same player can have separate sessions.
/// Sessions link SpacetimeDB identities to player UUIDs and track online status.
#[table(name = stdb_own_player_session_v1, public)]
#[derive(Debug, Clone)]
pub struct StdbOwnPlayerSessionV1 {
    #[primary_key]
    pub session_id: Identity,

    #[index(btree)]
    pub player_id: Uuid,

    pub is_online: bool,
}

/// Private player data table - contains full player information and timestamps.
///
/// Stores complete player records including authentication timestamps.
/// Display names must be unique across all players.
#[table(name = stdb_own_player_v1, public)]
#[derive(Debug, Clone)]
pub struct StdbOwnPlayerV1 {
    #[primary_key]
    pub player_id: Uuid,

    #[unique]
    pub display_name: String,
    pub avatar: String,

    pub created_at: Timestamp,

    pub signed_in_at: Timestamp,
    pub last_signed_out_at: Timestamp,
}

/// Public player card table - contains publicly visible player information.
///
/// Lightweight table for displaying player info without exposing private data
/// like timestamps. Automatically synced with the private player table.
#[table(name = stdb_pub_player_card_v1, public)]
#[derive(Debug, Clone)]
pub struct StdbPubPlayerCardV1 {
    #[primary_key]
    pub player_id: Uuid,

    pub display_name: String,
    pub avatar: String,
}

impl From<StdbOwnPlayerV1> for StdbPubPlayerCardV1 {
    fn from(player: StdbOwnPlayerV1) -> Self {
        Self {
            player_id: player.player_id,
            display_name: player.display_name,
            avatar: player.avatar,
        }
    }
}

#[reducer]
pub fn update_player_card_v1(ctx: &ReducerContext, display_name: String, avatar: String) -> ServiceResult<()> {
    let session = ctx.require_session()?;
    ctx.upsert_player_card(session.player_id, display_name, avatar)?;
    Ok(())
}
