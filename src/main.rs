use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use base64::decode;
use dotenv::dotenv;
use rusqlite::{Connection, params};
use serde::Deserialize;
use std::env;
use uuid::Uuid;

mod db;
mod handler;
mod models;

use db::{init};
use handler::{generate_url, pixel};
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    
    println!("Starting server at http://{}:{}", server_host, server_port);

    let app_state = web::Data::new(AppState {
        db: init(&database_url).expect("Failed to initialize database"),
    });

    HttpServer::new(move || {
        App::new()
            .service(generate_url)
            .service(pixel)
    })
        .bind(format!("{}:{}", server_host, server_port))?
        .run()
        .await
}
