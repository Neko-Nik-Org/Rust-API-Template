use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env::var as env_var;

pub struct PostgresState {
    pub db_pool: PgPool,
}

pub async fn init_postgres() -> PostgresState {
    // ENV variables
    const DEFAULT_MAX_POOL_SIZE: u32 = 2;

    // Read the pool size from the environment variable
    let max_pool_size: u32 = env_var("POSTGRES_DB_MAX_POOL_SIZE")
        .unwrap_or(DEFAULT_MAX_POOL_SIZE.to_string())  // Use default if the env variable is not set
        .parse()
        .expect("POSTGRES_DB_MAX_POOL_SIZE must be a number");

    // Create the pool using PgPoolOptions and set the max pool size
    let db_url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(max_pool_size)  // Set the pool size here
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    dbg!("Connected to the database!");

    PostgresState { db_pool }
}
