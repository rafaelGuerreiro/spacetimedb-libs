// TODO friends, guilds?, chat?, basic auth with game center and google play services?, i18n

use log::{debug, info};
use spacetimedb::ReducerContext;
use stdb_common::prelude::ServiceResult;

pub mod error;
pub mod player;
pub mod validate;

#[cfg(feature = "vip")]
pub mod vip;

pub mod prelude {
    pub use crate::{error::*, validate::*};
    pub use stdb_common::prelude::*;
}

#[inline]
pub fn stdb_init(ctx: &ReducerContext) -> ServiceResult<()> {
    player::stdb_init(ctx)?;

    #[cfg(feature = "vip")]
    vip::stdb_init(ctx)?;

    info!("stdb-player: initialized");
    Ok(())
}

#[inline]
pub fn stdb_identity_connected(ctx: &ReducerContext) -> ServiceResult<()> {
    player::stdb_identity_connected(ctx)?;

    #[cfg(feature = "vip")]
    vip::stdb_identity_connected(ctx)?;

    debug!("stdb-player: identity connected");
    Ok(())
}

#[inline]
pub fn stdb_identity_disconnected(ctx: &ReducerContext) {
    #[cfg(feature = "vip")]
    vip::stdb_identity_disconnected(ctx);

    player::stdb_identity_disconnected(ctx);

    debug!("stdb-player: identity disconnected");
}
