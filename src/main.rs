use actix_web::{web, App, HttpServer};

use dotenv::dotenv;


use std::env;


mod db;
mod handler;
mod models;

use db::{init};
use handler::{generate, track_pixel};
use models::AppState;
use crate::handler::{list_pixel_connections, list_pixels};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    println!("Starting server at http://{}:{}", server_host, server_port);

    let app_state = web::Data::new(AppState {
        db: init(&database_url).await.expect("Failed to initialize database"),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(generate)
            .service(track_pixel)
            .service(list_pixels)
            .service(list_pixel_connections)
    })
        .bind(format!("{}:{}", server_host, server_port))?
        .run()
        .await
}
