use spacetimedb::ReducerContext;
use stdb_common::prelude::*;

use crate::player::{PlayerSessionV1, PlayerV1, player_session_v1, player_v1};

pub trait PlayerExt {
    #[must_use]
    fn require_session(&self) -> ServiceResult<PlayerSessionV1>;

    #[must_use]
    fn require_player(&self, session: &PlayerSessionV1) -> ServiceResult<PlayerV1>;
}

impl PlayerExt for ReducerContext {
    fn require_session(&self) -> ServiceResult<PlayerSessionV1> {
        self.db
            .player_session_v1()
            .session_id()
            .find(self.sender)
            .ok_or(ServiceError::unauthorized())
    }

    fn require_player(&self, session: &PlayerSessionV1) -> ServiceResult<PlayerV1> {
        if session.session_id != self.sender {
            return Err(ServiceError::unauthorized());
        }

        let user = self
            .db
            .player_v1()
            .player_id()
            .find(&session.player_id)
            .ok_or(ServiceError::unauthorized())?;

        Ok(user)
    }
}
