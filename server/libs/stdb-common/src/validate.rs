use std::fmt::Display;

use crate::{
    error::{ErrorMapper, ServiceError, ServiceResult},
    uuid::Uuid,
};
use spacetimedb::ReducerContext;
use thiserror::Error;

#[must_use]
pub fn validate_str(name: impl Display, value: &str, min_length: u64, max_length: u64) -> ServiceResult<()> {
    let len = value.len() as u64;
    if min_length > 0 && value.is_empty() {
        Err(ValidationError::required_field(name))
    } else if len < min_length {
        Err(ValidationError::field_too_small(name, min_length))
    } else if len > max_length {
        Err(ValidationError::field_too_large(name, max_length))
    } else {
        Ok(())
    }
}

#[must_use]
pub fn validate_uuid(name: impl Display, uuid: &Uuid) -> ServiceResult<()> {
    // TODO unimplemented
    unimplemented!("validate size, dashes, and if it's not nil/max")
}

macro_rules! impl_validate_numeric {
    ($display:tt, $type:ty) => {
        #[must_use]
        pub fn $display(name: impl Display, value: $type, min_value: $type, max_value: $type) -> ServiceResult<()> {
            if value < min_value {
                Err(ValidationError::field_too_small(name, min_value))
            } else if value > max_value {
                Err(ValidationError::field_too_large(name, max_value))
            } else {
                Ok(())
            }
        }
    };
}

impl_validate_numeric!(validate_u8, u8);
impl_validate_numeric!(validate_u16, u16);
impl_validate_numeric!(validate_u32, u32);
impl_validate_numeric!(validate_u64, u64);
impl_validate_numeric!(validate_u128, u128);
impl_validate_numeric!(validate_usize, usize);

impl_validate_numeric!(validate_i8, i8);
impl_validate_numeric!(validate_i16, i16);
impl_validate_numeric!(validate_i32, i32);
impl_validate_numeric!(validate_i64, i64);
impl_validate_numeric!(validate_i128, i128);
impl_validate_numeric!(validate_isize, isize);

pub trait ValidateExt {
    #[must_use]
    fn require_private_access(&self) -> ServiceResult<()>;
}

impl ValidateExt for ReducerContext {
    fn require_private_access(&self) -> ServiceResult<()> {
        if self.sender != self.identity() {
            return Err(ServiceError::unauthorized());
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field '{0}' is required")]
    RequiredField(String),

    #[error("Field '{0}' must be at least {1}")]
    FieldTooSmall(String, String),

    #[error("Field '{0}' must be at most {1}")]
    FieldTooLarge(String, String),

    #[error("Field '{0}' must be a valid UUID")]
    InvalidUuid(String),
}

impl ValidationError {
    pub fn required_field(name: impl Display) -> ServiceError {
        ValidationError::RequiredField(name.to_string()).map_validation()
    }

    pub fn field_too_small(name: impl Display, min_length: impl Display) -> ServiceError {
        ValidationError::FieldTooSmall(name.to_string(), min_length.to_string()).map_validation()
    }

    pub fn field_too_large(name: impl Display, max_length: impl Display) -> ServiceError {
        ValidationError::FieldTooLarge(name.to_string(), max_length.to_string()).map_validation()
    }

    pub fn invalid_uuid(name: impl Display) -> ServiceError {
        ValidationError::InvalidUuid(name.to_string()).map_validation()
    }
}
