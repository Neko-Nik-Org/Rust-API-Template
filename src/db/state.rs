use sqlx::postgres::{PgPool, PgPoolOptions};
use actix_web::web::Data as webData;
use redis::Client as RedisClient;
use r2d2::Pool as RedisPool;
use std::env::var as env_var;
use log::info;

pub struct PostgresState {
    pub db_pool: PgPool,
}

pub struct RedisState {
    pub redis_pool: RedisPool<RedisClient>,
}


pub async fn init() -> (webData<PostgresState>, webData<RedisState>) {
    info!("Starting the server by initializing the logger and the database");
    // Initializers for the logger and the database
    env_logger::init(); // Initialize the logger to log all the logs

    // Initialize the Postgres client
    let postgres_state = init_postgres().await;

    // Initialize the Redis client
    let redis_state = init_redis().await;

    (webData::new(postgres_state), webData::new(redis_state))
}


async fn init_postgres() -> PostgresState {
    // Read the pool size from the environment variable
    let max_pool_size: u32 = env_var("POSTGRES_DB_MAX_POOL_SIZE")
        .unwrap_or("100".to_string()) // Default to 2 if not set
        .parse()
        .expect("POSTGRES_DB_MAX_POOL_SIZE must be a number");

    // Create the pool using PgPoolOptions and set the max pool size
    let db_url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(max_pool_size)  // Set the pool size here
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    info!("Successfully connected to the database");

    PostgresState { db_pool }
}


async fn init_redis() -> RedisState {
    // Create the Redis client
    let redis_url = env_var("REDIS_DB_URL").expect("REDIS_DB_URL must be set");
    let max_pool_size: u32 = env_var("REDIS_DB_MAX_POOL_SIZE")
        .unwrap_or("50".to_string()) // Default to 50 if not set
        .parse()
        .expect("REDIS_DB_MAX_POOL_SIZE must be a number");
    let redis_client = RedisClient::open(redis_url).unwrap();

    let pool = RedisPool::builder()
        .max_size(max_pool_size)
        .build(redis_client)
        .unwrap();
    
    info!("Successfully connected to the Redis server");

    RedisState { redis_pool: pool }
}
