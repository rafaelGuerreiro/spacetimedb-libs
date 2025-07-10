use stdb_common::prelude::{ErrorMapper, ServiceError, Uuid};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error("Player '{0}' not found")]
    PlayerNotFound(Uuid),
}

impl PlayerError {
    pub fn player_not_found(uuid: Uuid) -> ServiceError {
        Self::PlayerNotFound(uuid).map_validation()
    }
}
