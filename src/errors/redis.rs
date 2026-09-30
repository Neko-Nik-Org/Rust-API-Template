use redis::RedisError;
use std::fmt;



#[derive(Debug)]
pub enum RedisAppError {
    Redis(RedisError),
}


// ------- Implementations ------- //


impl fmt::Display for RedisAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedisAppError::Redis(e) => write!(
                f,
                "Redis error: {} | kind={:?}",
                e,
                e.kind()
            )
        }
    }
}


impl From<RedisError> for RedisAppError {
    fn from(e: RedisError) -> Self {
        RedisAppError::Redis(e)
    }
}


impl std::error::Error for RedisAppError {}
