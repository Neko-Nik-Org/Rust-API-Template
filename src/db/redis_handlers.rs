use r2d2::PooledConnection as RedisPooledConnection;
use serde::{Deserialize, Serialize};
use redis::Client as RedisClient;
use r2d2::Pool as RedisPool;
use redis::Commands;
use log::trace;

#[derive(Deserialize, Serialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_data: String, // Its a JSON string
}

// Create a simple UUID for the session
fn create_session_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub async fn create_session(redis_pool: &RedisPool<RedisClient>, user_data: String) -> Result<String, redis::RedisError> {
    let session_id = create_session_id();
    trace!("Creating a new session with ID: {}", session_id);
    let session_info = SessionInfo {
        session_id: session_id.clone(),
        user_data,
    };

    // Convert the session_info struct to a JSON string
    let session_info_json = serde_json::to_string(&session_info).unwrap();

    // Get a Redis connection from the pool
    let mut conn: RedisPooledConnection<RedisClient> = redis_pool.get().unwrap();

    // Explicitly specify the types for set
    conn.set::<String, String, ()>(session_id.clone(), session_info_json)?;

    // Return the session ID
    Ok(session_id)
}


pub async fn health_check(redis_pool: &RedisPool<RedisClient>) -> Result<String, redis::RedisError> {
    let mut conn: RedisPooledConnection<RedisClient> = redis_pool.get().unwrap();
    let pong: String = conn.ping()?;

    Ok(pong)
}
