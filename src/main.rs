use actix_web::web::scope as actix_scope;
use actix_web::{App, HttpServer};
use std::env::var as env_var;
use routes::sample_db;
use actix_cors::Cors;
use routes::health;

mod routes;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (postgres_data, redis_data) = db::state::init().await;

    // Start the Actix web server
    HttpServer::new(move || {
        App::new().app_data(postgres_data.clone()).app_data(redis_data.clone())
            .wrap(Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(120)
            )
            .service(
                actix_scope("/health")
                .service(health::api_health_check)
                .service(health::db_health_check)
                .service(health::redis_health_check)
            )
            .service(
                actix_scope("/sample_db")
                // .guard() ? what is it?
                .service(sample_db::create_note_handler)
                .service(sample_db::list_notes_handler)
                .service(sample_db::create_session_handler)
            )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("SERVER_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
