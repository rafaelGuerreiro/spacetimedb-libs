// TODO friends, guilds?, chat?, basic auth with game center and google play services?, i18n

pub(crate) mod player;
pub(crate) mod validate;

pub mod prelude {
    pub use crate::{player::*, validate::*};
    pub use stdb_common::prelude::*;
}
