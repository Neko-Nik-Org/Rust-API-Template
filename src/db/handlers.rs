use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::db::state::PostgresState;
use sqlx::Row;


#[derive(Deserialize, Serialize)]
pub struct Note {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub title: String,
    pub content: String,
}


// Check if DB is workign as expected
#[get("/health")]
pub async fn db_health_check(
    state: web::Data<PostgresState>,
) -> impl Responder {
    let result = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|err| err.to_string());

    match result {
        Ok(_) => HttpResponse::Ok().json("Database is healthy!"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}


// POST /postgres/create-note
#[post("/create-note")]
pub async fn create_note_handler(
    body: web::Json<Note>,         // The request body
    state: web::Data<PostgresState>, // The state containing the DB pool
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO notes (title, content)
        VALUES ($1, $2)
        "#
    )
    .bind(&body.title)
    .bind(&body.content)
    .execute(&state.db_pool)
    .await
    .map_err(|err| err.to_string());

    match result {
        Ok(_) => HttpResponse::Ok().json("Note created successfully!"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}

// GET /postgres/notes
#[get("/notes")]
pub async fn list_notes_handler(state: web::Data<PostgresState>) -> impl Responder {
    let result = sqlx::query(
        r#"
        SELECT id, title, content FROM notes
        "#
    )
    .map(|row: sqlx::postgres::PgRow| Record {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
    })
    .fetch_all(&state.db_pool)
    .await
    .map_err(|err| err.to_string());

    match result {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}
