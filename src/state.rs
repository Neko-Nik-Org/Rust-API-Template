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
    let url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");

    let mut cfg: Config = url.parse::<Config>().expect("invalid POSTGRES_DB_URL");
    cfg.application_name("rust-api");
    cfg.connect_timeout(Duration::from_secs(5));

    cfg
}


fn init_pg_pool() -> PgPool {
    let cfg: Config = build_pg_config();

    let mgr = Manager::from_config(
        cfg,
        NoTls,
        deadpool_postgres::ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    let max = env_var("PG_POOL_MAX")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);

    let pool = PgPool::builder(mgr)
        .max_size(max)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts {
            // how long to wait for an idle connection from the pool
            wait: Some(Duration::from_secs(5)),
            // how long to spend creating a new connection (if pool can grow)
            create: Some(Duration::from_secs(5)),
            // how long to spend recycling/validating a connection
            recycle: Some(Duration::from_secs(5)),
        })
        .build()
        .expect("failed to build pg pool");

    info!("Postgres pool initialized (max_size={max})");
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
