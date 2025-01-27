use actix_web::{get, web, HttpResponse, Responder};
use crate::db::pgsql_handlers::health_check as check_db;
use crate::db::state::PostgresState;


// Health check endpoint
#[get("/api")]
async fn api_health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}


#[get("/db")]
async fn db_health_check(
    state: web::Data<PostgresState>,
) -> impl Responder {
    match check_db(&state.db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Database is running!"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}
