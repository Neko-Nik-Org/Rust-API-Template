use crate::db::pgsql_handlers::health_check as check_db;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;


// Health check endpoint
#[get("/api")]
async fn api_health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}


// Database health check
#[get("/pgsql")]
async fn db_health_check(state: web::Data<PgPool>) -> impl Responder {
    match check_db(&state).await {
        Ok(_) => HttpResponse::Ok().body("Database is running!"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}
