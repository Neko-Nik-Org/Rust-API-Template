use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env::var as env_var;
use log::info;

pub struct PostgresState {
    pub db_pool: PgPool,
}

pub async fn init_postgres() -> PostgresState {
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
