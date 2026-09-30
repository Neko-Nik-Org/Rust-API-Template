use super::redis::RedisAppError;
use super::pgsql::PgError;
use std::fmt;


pub type AppResult<T> = Result<T, AppError>;


#[derive(Debug)]
pub enum AppError {
    SerDe(serde_json::Error),
    Reqwest(reqwest::Error),
    Redis(RedisAppError),
    Pgsql(PgError),

    PreconditionFailed(String),
    BadRequest(String),
    Unprocessable(String),
    NotFound(String),
    Gone(String),
}


// ------- Implementations ------- //


impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::SerDe(e) => write!(f, "JSON: {}", e),
            AppError::Reqwest(e) => write!(f, "Reqwest: {}", e),
            AppError::Pgsql(e) => write!(f, "PostgreSQL: {}", e),
            AppError::Redis(e) => write!(f, "Redis: {}", e),
            AppError::BadRequest(s) => write!(f, "Bad Request: {}", s),
            AppError::NotFound(s) => write!(f, "Resource not found: {}", s),
            AppError::Gone(s) => write!(f, "It's gone: {}", s),
            AppError::PreconditionFailed(s) => write!(f, "Precondition Failed: {}", s),
            AppError::Unprocessable(s) => write!(f, "Unprocessable: {}", s),
        }
    }
}


impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerDe(e)
    }
}


impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Reqwest(e)
    }
}


impl From<RedisAppError> for AppError {
    fn from(e: RedisAppError) -> Self {
        AppError::Redis(e)
    }
}


impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::Redis(RedisAppError::from(e))
    }
}


impl From<PgError> for AppError {
    fn from(e: PgError) -> Self {
        AppError::Pgsql(e)
    }
}


impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Gone(s)
    }
}


impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Gone(s.to_string())
    }
}


impl std::error::Error for AppError {}
