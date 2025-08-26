use sqlx::postgres::{PgPool, PgPoolOptions};
use actix_web::web::Data as webData;
use std::env::var as env_var;
use super::types::AppCache;
use std::time::Duration;
use actix_web::web;
use log::info;


async fn init_postgres() -> PgPool {
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

    db_pool
}


fn init_cache() -> AppCache {
    // Get the max capacity from the environment variable
    let max_capacity: u64 = env_var("CACHE_MAX_CAPACITY")
        .unwrap_or("100_000".to_string())
        .parse()
        .expect("CACHE_MAX_CAPACITY must be a number");

    // Max Cache TTL, get from the environment variable
    let time_to_live: u64 = env_var("CACHE_TIME_TO_LIVE")
        .unwrap_or("300".to_string())
        .parse()
        .expect("CACHE_TIME_TO_LIVE must be a number");

    // Build the AppCache
    let cache: AppCache = AppCache::builder()
        .max_capacity(max_capacity)
        .time_to_live(Duration::from_secs(time_to_live))
        .build();

    cache
}

pub async fn init() -> (webData<PgPool>, web::Data<AppCache>) {
    info!("Starting the server by initializing the logger and the in-memory cache");
    // Initializers for the logger and the database
    env_logger::init(); // Initialize the logger to log all the logs

    // Initialize the Postgres client
    let postgres_state = init_postgres().await;

    // Initialize the in-memory cache (Moka)
    let in_mem_cache = init_cache();

    // Wrap the state of the application and share it
    (webData::new(postgres_state), webData::new(in_mem_cache))
}
