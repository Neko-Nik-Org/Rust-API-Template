use crate::db::pgsql_handlers::{Note, add_new_notes, fetch_all_notes};
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::state::{PostgresState, RedisState};
use crate::db::redis_handlers::create_session;


#[post("/create-note")]
pub async fn create_note_handler(
    body: web::Json<Note>,         // The request body
    state: web::Data<PostgresState>, // The state containing the DB pool
) -> impl Responder {
    match add_new_notes(&state.db_pool, vec![body.into_inner()]).await {
        Ok(_) => HttpResponse::Ok().json("Note created successfully!"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}


#[get("/notes")]
pub async fn list_notes_handler(state: web::Data<PostgresState>) -> impl Responder {
    match fetch_all_notes(&state.db_pool).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/create-session")]
pub async fn create_session_handler(
    body: String,         // The request body (For now accept anything)
    state: web::Data<RedisState>, // The state containing the DB pool
) -> impl Responder {
    match create_session(&state.redis_pool, body).await {
        Ok(session_id) => HttpResponse::Ok().json(session_id),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}
