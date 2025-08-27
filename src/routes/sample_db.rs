use crate::db::pgsql_handlers::{Note, add_new_notes, fetch_all_notes};
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::types::{AppCache, make_key};
use deadpool_postgres::Pool as PgPool;



async fn cache_data<T, S>(key: S, value: &T, cache: &AppCache)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
        S: Into<String>
{
    // Serialize the value to a JSON string
    let json = serde_json::to_string(value).unwrap();

    // Insert the JSON string into the cache
    cache.insert(make_key(key), json).await;
}

#[post("/create-note")]
pub async fn create_note_handler(
    body: web::Json<Note>,         // The request body
    pg_pool: web::Data<PgPool>, // The state containing the DB pool
) -> impl Responder {
    match add_new_notes(&pg_pool, vec![body.into_inner()]).await {
        Ok(_) => HttpResponse::Ok().json("Note created successfully!"),
        Err(err) => HttpResponse::ExpectationFailed().json(format!("Failed: {}", err)),
    }
}


#[get("/notes")]
pub async fn list_notes_handler(
    pg_pool: web::Data<PgPool>,
    cache: web::Data<AppCache>
) -> impl Responder {
    let cache_key = "list_notes_handler";

    // If cache is present then serve from cache
    if let Some(cached_notes) = cache.get(&make_key(cache_key)).await {
        return HttpResponse::Ok()
            .insert_header(("X-Cache", "HIT"))
            .json(serde_json::from_str::<Vec<Note>>(&cached_notes).unwrap());
    }

    match fetch_all_notes(&pg_pool).await {
        Ok(notes) => {
            cache_data(cache_key, &notes, &cache).await;
            HttpResponse::Ok().json(notes)
        }
        Err(_) => HttpResponse::ExpectationFailed().finish(),
    }
}


#[post("/create-session")]
pub async fn create_session_handler(
    body: String,         // The request body (For now accept anything)
    state: web::Data<AppCache>, // The state containing the Cache
) -> impl Responder {
    // Generate a new session ID
    let session_id = uuid::Uuid::new_v4().to_string();

    state.insert(make_key(session_id.clone()), body).await;

    HttpResponse::Ok().body(session_id)
}


#[get("/get-session/{session_id}")]
pub async fn get_session_handler(
    session_id: web::Path<String>,
    state: web::Data<AppCache>,
) -> impl Responder {
    let key = make_key(session_id.clone());

    if let Some(value) = state.get(&key).await {
        HttpResponse::Ok()
            .insert_header(("Cache-Control", "cache"))
            .body(value)
    } else {
        HttpResponse::NotFound().finish()
    }
}
