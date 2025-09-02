use crate::db::pgsql_handlers::health_check as health_check_pgsql;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::types::{AppCache, make_key};
use deadpool_postgres::Pool as PgPool;
use std::sync::mpsc::Sender;


// Health check endpoint
#[get("/api")]
async fn api_health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}


// Database health check
#[get("/pgsql")]
async fn db_health_check(state: web::Data<PgPool>) -> impl Responder {
    match health_check_pgsql(&state).await {
        Ok(_) => HttpResponse::Ok().body("Database is running!"),
        Err(err) => HttpResponse::PreconditionFailed().json(format!("Failed: {}", err)),
    }
}


// Cache health check
#[get("/cache")]
async fn cache_health_check(cache: web::Data<AppCache>) -> impl Responder {
    const CACHE_KEY: &str = "health_check";
    const CACHE_VALUE: &str = "Cache is running!";
    let key = make_key(CACHE_KEY);

    cache.insert(key, CACHE_VALUE.to_string()).await;

    if let Some(cached_value) = cache.get(&make_key(CACHE_KEY)).await {
        if cached_value == CACHE_VALUE {
            return HttpResponse::Ok().body(cached_value);
        }
    }
    HttpResponse::PreconditionFailed().body("Cache health check failed!")
}


// Channel Health check
#[post("/channel")]
async fn channel_health_check(state: web::Data<Sender<u8>>) -> impl Responder {
    // Send a 7 int to the channel
    let _ = state.send(7);
    HttpResponse::Ok().body("Channel health check initiated!")
}
