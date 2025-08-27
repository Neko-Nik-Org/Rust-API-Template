use deadpool_postgres::{Manager, RecyclingMethod, Pool as PgPool};
use deadpool::{managed::Timeouts, Runtime};
use actix_web::web::Data as webData;
use tokio_postgres::{Config, NoTls};
use std::env::var as env_var;
use super::types::AppCache;
use std::time::Duration;
use actix_web::web;
use log::info;


struct PgSettings {
    url: String,
    conn_timeout: u64,
    max_pool_size: usize,
    idle_timeout: u64,
    new_connection_timeout: u64,
    recycle_timeout: u64,
}


struct MokaSettings {
    cache_size: u64,
    expiration_time: Duration,
}


struct AppSettings {
    pg_settings: PgSettings,
    cache_settings: MokaSettings,
    enable_logging: bool,
}


trait FromEnv {
    fn from_env() -> Self;
}


impl FromEnv for PgSettings {
    fn from_env() -> Self {
        let url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");
        let conn_timeout = env_var("POSTGRES_CONN_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("POSTGRES_CONN_TIMEOUT must be a positive integer of type u64");
        let max_pool_size = env_var("PG_POOL_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_MAX_SIZE must be a positive integer of type usize");
        let idle_timeout = env_var("PG_POOL_IDLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_IDLE_TIMEOUT must be a positive integer of type u64");
        let new_connection_timeout = env_var("PG_POOL_NEW_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_NEW_CONNECTION_TIMEOUT must be a positive integer of type u64");
        let recycle_timeout = env_var("PG_POOL_RECYCLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_RECYCLE_TIMEOUT must be a positive integer of type u64");

        PgSettings {
            url,
            conn_timeout,
            max_pool_size,
            idle_timeout,
            new_connection_timeout,
            recycle_timeout,
        }
    }
}


impl FromEnv for MokaSettings {
    fn from_env() -> Self {
        let cache_size = env_var("CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_SIZE must be a positive integer of type u64");
        let expiration_time = env_var("CACHE_EXPIRATION_TIME")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_EXPIRATION_TIME must be a positive integer of type u64");

        MokaSettings {
            cache_size,
            expiration_time: Duration::from_secs(expiration_time),
        }
    }
}


impl FromEnv for AppSettings {
    fn from_env() -> Self {
        let enable_logging = env_var("ENABLE_LOGGING").expect("ENABLE_LOGGING must be set as true or false");
        let enable_logging = match enable_logging.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("ENABLE_LOGGING must be set as true or false"),
        };

        AppSettings {
            pg_settings: PgSettings::from_env(),
            cache_settings: MokaSettings::from_env(),
            enable_logging,
        }
    }
}


fn build_pg_config(settings: &PgSettings) -> Config {
    // Initialize the Postgres configuration
    let mut cfg: Config = settings.url.parse::<Config>().expect("invalid POSTGRES_DB_URL");
    cfg.application_name("rust-api");
    cfg.connect_timeout(Duration::from_secs(settings.conn_timeout));

    cfg
}


fn init_pg_pool(pg_settings: &PgSettings) -> PgPool {
    // Get the Postgres base configuration
    let cfg: Config = build_pg_config(pg_settings);
    let mgr = Manager::from_config(
        cfg,
        NoTls,
        deadpool_postgres::ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    let pool = PgPool::builder(mgr)
        .max_size(pg_settings.max_pool_size)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts {
            // how long to wait for an idle connection from the pool
            wait: Some(Duration::from_secs(pg_settings.idle_timeout)),
            // how long to spend creating a new connection (if pool can grow)
            create: Some(Duration::from_secs(pg_settings.new_connection_timeout)),
            // how long to spend recycling/validating a connection
            recycle: Some(Duration::from_secs(pg_settings.recycle_timeout)),
        })
        .build()
        .expect("failed to build pg pool");

    info!("Postgres pool initialized (max_pool_size={})", pg_settings.max_pool_size);
    pool
}


fn init_cache(cache_settings: &MokaSettings) -> AppCache {
    // Build the AppCache
    let cache: AppCache = AppCache::builder()
        .max_capacity(cache_settings.cache_size)
        .time_to_live(cache_settings.expiration_time)
        .build();

    info!("In-memory cache initialized (max_capacity={})", cache_settings.cache_size);
    cache
}


pub async fn init() -> (webData<PgPool>, web::Data<AppCache>) {
    // Preparing to start the server by collecting environment variables
    let app_settings = AppSettings::from_env();

    if app_settings.enable_logging {
        env_logger::init(); // Initialize the logger to log all the logs
        info!("Starting the server by initializing the application state");
    }

    // Initialize the Postgres client
    let postgres_state = init_pg_pool(&app_settings.pg_settings);

    // Initialize the in-memory cache (Moka)
    let in_mem_cache = init_cache(&app_settings.cache_settings);

    // Wrap the state of the application and share it
    (webData::new(postgres_state), webData::new(in_mem_cache))
}
