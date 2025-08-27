use deadpool_postgres::{Manager, RecyclingMethod, Pool as PgPool};
use actix_web::web::Data as webData;
use tokio_postgres::{Config, NoTls};
use deadpool::{managed::Timeouts, Runtime};
use std::env::var as env_var;
use super::types::AppCache;
use std::time::Duration;
use actix_web::web;
use log::info;


fn build_pg_config() -> Config {
    let url: String = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");
    let conn_timeout: u64 = env_var("POSTGRES_CONN_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .expect("POSTGRES_CONN_TIMEOUT must be a positive integer of type u64");

    // Initialize the Postgres configuration
    let mut cfg: Config = url.parse::<Config>().expect("invalid POSTGRES_DB_URL");
    cfg.application_name("rust-api");
    cfg.connect_timeout(Duration::from_secs(conn_timeout));

    cfg
}


fn init_pg_pool() -> PgPool {
    let max_pool_size: usize = env_var("PG_POOL_MAX_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .expect("PG_POOL_MAX_SIZE must be a positive integer of type usize");
    let idle_timeout: u64 = env_var("PG_POOL_IDLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .expect("PG_POOL_IDLE_TIMEOUT must be a positive integer of type u64");
    let new_connection_timeout: u64 = env_var("PG_POOL_NEW_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .expect("PG_POOL_NEW_CONNECTION_TIMEOUT must be a positive integer of type u64");
    let recycle_timeout: u64 = env_var("PG_POOL_RECYCLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .expect("PG_POOL_RECYCLE_TIMEOUT must be a positive integer of type u64");

    // Get the Postgres base configuration
    let cfg: Config = build_pg_config();
    let mgr = Manager::from_config(
        cfg,
        NoTls,
        deadpool_postgres::ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    let pool = PgPool::builder(mgr)
        .max_size(max_pool_size)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts {
            // how long to wait for an idle connection from the pool
            wait: Some(Duration::from_secs(idle_timeout)),
            // how long to spend creating a new connection (if pool can grow)
            create: Some(Duration::from_secs(new_connection_timeout)),
            // how long to spend recycling/validating a connection
            recycle: Some(Duration::from_secs(recycle_timeout)),
        })
        .build()
        .expect("failed to build pg pool");

    info!("Postgres pool initialized (max_size={max_pool_size})");
    pool
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
    let postgres_state = init_pg_pool();

    // Initialize the in-memory cache (Moka)
    let in_mem_cache = init_cache();

    // Wrap the state of the application and share it
    (webData::new(postgres_state), webData::new(in_mem_cache))
}
