use actix_web::{App, HttpServer, HttpResponse, Responder, get};
use actix_web::web::Data as webData;
use std::env::var as env_var;

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Postgres state
    let postgres_state = db::state::init_postgres().await;
    let postgres_data = webData::new(postgres_state);

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(postgres_data.clone())
            .service(health_check)
            .service(
                actix_web::web::scope("/postgres") // PostgreSQL endpoints
                    .service(db::handlers::create_note_handler)
                    .service(db::handlers::list_notes_handler),
            )
    })
        // .bind(("0.0.0.0", 8686))?
        .bind(("127.0.0.1", 8686))?
        .workers(env_var("SERVER_WORKERS_COUNT").unwrap_or("1".to_string()).parse().unwrap())
    .run()
    .await
}

// Health check endpoint
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}
