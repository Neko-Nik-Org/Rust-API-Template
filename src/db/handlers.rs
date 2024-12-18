use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::db::state::PostgresState;

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

// POST /postgres/create-note
#[post("/create-note")]
pub async fn create_note_handler(
    body: web::Json<Note>,         // The request body
    state: web::Data<PostgresState>, // The state containing the DB pool
) -> impl Responder {
    // Insert query with placeholders for $1, $2
    let result = sqlx::query!(
        r#"
        INSERT INTO notes (title, content)
        VALUES ($1, $2)
        "#,
        body.title,
        body.content
    )
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
    // Fetch all notes from the database
    let result = sqlx::query_as!(
        Record,  // Return type of the query
        "SELECT id, title, content FROM notes"
    )
    .fetch_all(&state.db_pool)
    .await;

    match result {
        Ok(notes) => HttpResponse::Ok().json(notes), // Return as JSON
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed: {}", err)),
    }
}
