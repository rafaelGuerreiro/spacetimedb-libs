use crate::{prelude::PlayerExt, vip::repository::VipRepository};
use spacetimedb::{Filter, ReducerContext, SpacetimeType, Timestamp, client_visibility_filter, reducer, table};
use std::u64;
use stdb_common::prelude::{ServiceResult, Uuid};

pub mod repository;

pub(crate) fn stdb_init(_ctx: &ReducerContext) -> ServiceResult<()> {
    Ok(())
}

pub(crate) fn stdb_identity_connected(_ctx: &ReducerContext) -> ServiceResult<()> {
    Ok(())
}

pub(crate) fn stdb_identity_disconnected(_ctx: &ReducerContext) {}

#[client_visibility_filter]
const STDB_OWN_VIP_LIST_V1_FILTER: Filter = Filter::Sql(
    r#"
    select v.*
    from stdb_own_vip_v1 v
    join stdb_own_player_session_v1 s
        on s.player_id = v.sender_id
"#,
);

#[table(
    name = stdb_own_vip_v1,
    public,
    index(name = player_ids_index, btree(columns = [sender_id, receiver_id])),
)]
#[derive(Debug, Clone)]
pub struct StdbOwnVipV1 {
    #[auto_inc]
    #[primary_key]
    pub vip_id: u64,

    #[index(btree)]
    pub sender_id: Uuid,

    // TODO think about a request limit to avoid harassment, blocking, etc.
    pub receiver_id: Uuid,

    /// Tags are used by the player to just categorize this VIP connection.
    /// It could be an emoji that's shown besides the VIP name.
    pub tag: String,

    pub status: VipStatusV1,

    pub created_at: Timestamp,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, SpacetimeType)]
pub enum VipStatusV1 {
    /// Added another player, but not yet accepted
    InviteSent,

    /// Received from another player, but didn't accept yet
    InviteReceived,

    /// Both players added each other as friends
    Friends,
}

#[reducer]
pub fn insert_vip_v1(ctx: &ReducerContext, receiver_id: Uuid, tag: String) -> ServiceResult<()> {
    let session = ctx.require_session()?;
    ctx.insert_vip(session.player_id, receiver_id, tag)?;
    Ok(())
}
