use spacetimedb::{Filter, Identity, ReducerContext, client_visibility_filter, reducer, table};
use stdb_common::prelude::{ResultExt, ServiceResult, Uuid, validate_str};

use crate::prelude::PlayerExt;

#[client_visibility_filter]
const AUTH_SESSION_V1_FILTER: Filter = Filter::Sql(
    r#"
    select s.*
    from player_session_v1 s
    where s.session_id = :sender
"#,
);

#[table(name = player_session_v1, public)]
#[derive(Debug, Clone)]
pub struct PlayerSessionV1 {
    #[primary_key]
    pub session_id: Identity,

    pub player_id: Uuid,
}

#[table(name = player_v1, public)]
#[derive(Debug, Clone)]
pub struct PlayerV1 {
    #[primary_key]
    pub player_id: Uuid,

    #[unique]
    pub display_name: String,

    pub avatar: String,
}

#[reducer]
pub fn insert_or_update_player_v1(ctx: &ReducerContext, display_name: String, avatar: String) -> ServiceResult<()> {
    validate_str("display_name", &display_name, 8, 64)?;
    validate_str("avatar", &avatar, 8, 64)?;

    let session = ctx.require_session()?;

    let player = PlayerV1 {
        player_id: session.player_id,
        display_name,
        avatar,
    };

    ctx.db
        .player_v1()
        .player_id()
        .try_insert_or_update(player)
        .map_conflict_ctx("failed to insert or update player")?;
    Ok(())
}
