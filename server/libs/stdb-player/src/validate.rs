use crate::player::{
    StdbOwnPlayerSessionV1, StdbOwnPlayerV1,
    repository::{PlayerRepository, PlayerSessionRepository},
};
use spacetimedb::ReducerContext;
use stdb_common::prelude::{ServiceError, ServiceResult};

/// Extension trait for player validation and authorization operations.
///
/// Provides methods to validate session ownership and ensure players exist
/// before performing sensitive operations. All methods return authorization
/// errors when validation fails.
pub trait PlayerExt {
    /// Requires that the current sender has a valid session.
    ///
    /// Validates that the sender identity has an active session in the database.
    /// This is typically used as the first step in player-related operations
    /// to ensure the request comes from an authenticated user.
    ///
    /// # Errors
    /// Returns `ServiceError::unauthorized()` if no session exists for the sender.
    #[must_use]
    fn require_session(&self) -> ServiceResult<StdbOwnPlayerSessionV1>;

    /// Requires that the session belongs to the current sender and has a valid player.
    ///
    /// Validates that:
    /// 1. The provided session belongs to the current sender (prevents session hijacking)
    /// 2. The session's player exists in the database
    ///
    /// Used to ensure the caller owns the session and the associated player exists.
    ///
    /// # Errors
    /// Returns `ServiceError::unauthorized()` if:
    /// - Session doesn't belong to the current sender
    /// - No player exists for the session's player ID
    #[must_use]
    fn require_player(&self, session: &StdbOwnPlayerSessionV1) -> ServiceResult<StdbOwnPlayerV1>;
}

impl PlayerExt for ReducerContext {
    fn require_session(&self) -> ServiceResult<StdbOwnPlayerSessionV1> {
        self.find_session(self.sender).ok_or(ServiceError::unauthorized())
    }

    fn require_player(&self, session: &StdbOwnPlayerSessionV1) -> ServiceResult<StdbOwnPlayerV1> {
        if session.session_id != self.sender {
            return Err(ServiceError::unauthorized());
        }

        self.find_player(&session.player_id).ok_or(ServiceError::unauthorized())
    }
}
