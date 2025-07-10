use std::{error::Error as StdError, fmt::Display};
use thiserror::Error;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    /// 400 Bad Request - Client sent an invalid request
    #[error("E400: {0}")]
    BadRequest(String),

    /// 401 Unauthorized - Client needs to authenticate
    #[error("E401: {0}")]
    Unauthorized(String),

    /// 403 Forbidden - Client doesn't have access rights
    #[error("E403: {0}")]
    Forbidden(String),

    /// 404 Not Found - Resource not found
    #[error("E404: {0}")]
    NotFound(String),

    /// 409 Conflict - Request couldn't be completed due to conflict
    #[error("E409: {0}")]
    Conflict(String),

    /// 418 I'm a teapot - Used for validation errors
    #[error("E418: {0}")]
    Validation(String),

    /// 429 Too Many Requests - Rate limiting
    #[error("E429: {0}")]
    RateLimited(String),

    /// 500 Internal Server Error - Server encountered an unexpected condition
    #[error("E500: {0}")]
    Internal(String),
}

impl ServiceError {
    pub fn unauthorized() -> Self {
        ServiceError::Unauthorized("Unauthorized".to_string())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        ServiceError::Internal(message.into())
    }
}

/// Trait to provide a fluent API for mapping domain-specific errors to ServiceError
pub trait ErrorMapper {
    /// Maps the error to ServiceError::BadRequest
    fn map_bad_request(self) -> ServiceError;
    fn map_bad_request_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::Unauthorized
    fn map_unauthorized(self) -> ServiceError;
    fn map_unauthorized_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::Forbidden
    fn map_forbidden(self) -> ServiceError;
    fn map_forbidden_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::NotFound
    fn map_not_found(self) -> ServiceError;
    fn map_not_found_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::Conflict
    fn map_conflict(self) -> ServiceError;
    fn map_conflict_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::Validation
    fn map_validation(self) -> ServiceError;
    fn map_validation_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::RateLimited
    fn map_rate_limited(self) -> ServiceError;
    fn map_rate_limited_ctx(self, error_ctx: impl Display) -> ServiceError;

    /// Maps the error to ServiceError::Internal
    fn map_internal(self) -> ServiceError;
    fn map_internal_ctx(self, error_ctx: impl Display) -> ServiceError;
}

impl<E> ErrorMapper for E
where
    E: StdError + Send + Sync + 'static,
{
    fn map_bad_request(self) -> ServiceError {
        ServiceError::BadRequest(self.to_string())
    }

    fn map_bad_request_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::BadRequest(format!("{}: {}", error_ctx, self))
    }

    fn map_unauthorized(self) -> ServiceError {
        ServiceError::Unauthorized(self.to_string())
    }

    fn map_unauthorized_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::Unauthorized(format!("{}: {}", error_ctx, self))
    }

    fn map_forbidden(self) -> ServiceError {
        ServiceError::Forbidden(self.to_string())
    }

    fn map_forbidden_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::Forbidden(format!("{}: {}", error_ctx, self))
    }

    fn map_not_found(self) -> ServiceError {
        ServiceError::NotFound(self.to_string())
    }

    fn map_not_found_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::NotFound(format!("{}: {}", error_ctx, self))
    }

    fn map_conflict(self) -> ServiceError {
        ServiceError::Conflict(self.to_string())
    }

    fn map_conflict_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::Conflict(format!("{}: {}", error_ctx, self))
    }

    fn map_validation(self) -> ServiceError {
        ServiceError::Validation(self.to_string())
    }

    fn map_validation_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::Validation(format!("{}: {}", error_ctx, self))
    }

    fn map_rate_limited(self) -> ServiceError {
        ServiceError::RateLimited(self.to_string())
    }

    fn map_rate_limited_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::RateLimited(format!("{}: {}", error_ctx, self))
    }

    fn map_internal(self) -> ServiceError {
        ServiceError::Internal(self.to_string())
    }

    fn map_internal_ctx(self, error_ctx: impl Display) -> ServiceError {
        ServiceError::Internal(format!("{}: {}", error_ctx, self))
    }
}

/// Extension trait for Result<T, E> to make error mapping more ergonomic
pub trait ResultExt<T, E: StdError + Send + Sync + 'static> {
    /// Maps an error to ServiceError::BadRequest
    fn map_bad_request(self) -> ServiceResult<T>;
    fn map_bad_request_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::Unauthorized
    fn map_unauthorized(self) -> ServiceResult<T>;
    fn map_unauthorized_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::Forbidden
    fn map_forbidden(self) -> ServiceResult<T>;
    fn map_forbidden_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::NotFound
    fn map_not_found(self) -> ServiceResult<T>;
    fn map_not_found_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::Conflict
    fn map_conflict(self) -> ServiceResult<T>;
    fn map_conflict_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::Validation
    fn map_validation(self) -> ServiceResult<T>;
    fn map_validation_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::RateLimited
    fn map_rate_limited(self) -> ServiceResult<T>;
    fn map_rate_limited_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;

    /// Maps an error to ServiceError::Internal
    fn map_internal(self) -> ServiceResult<T>;
    fn map_internal_ctx(self, error_ctx: impl Display) -> ServiceResult<T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn map_bad_request(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_bad_request())
    }

    fn map_bad_request_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_bad_request_ctx(error_ctx))
    }

    fn map_unauthorized(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_unauthorized())
    }

    fn map_unauthorized_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_unauthorized_ctx(error_ctx))
    }

    fn map_forbidden(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_forbidden())
    }

    fn map_forbidden_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_forbidden_ctx(error_ctx))
    }

    fn map_not_found(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_not_found())
    }

    fn map_not_found_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_not_found_ctx(error_ctx))
    }

    fn map_conflict(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_conflict())
    }

    fn map_conflict_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_conflict_ctx(error_ctx))
    }

    fn map_validation(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_validation())
    }

    fn map_validation_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_validation_ctx(error_ctx))
    }

    fn map_rate_limited(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_rate_limited())
    }

    fn map_rate_limited_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_rate_limited_ctx(error_ctx))
    }

    fn map_internal(self) -> ServiceResult<T> {
        self.map_err(|e| e.map_internal())
    }

    fn map_internal_ctx(self, error_ctx: impl Display) -> ServiceResult<T> {
        self.map_err(|e| e.map_internal_ctx(error_ctx))
    }
}
