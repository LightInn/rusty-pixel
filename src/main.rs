use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, HttpRequest};
use std::env;
use base64::decode;
use serde::Deserialize;
use uuid::{Uuid};

#[derive(Deserialize)]
struct Info {
    uuid: String,
}


#[get("/generate-url")]
async fn generate_url() -> impl Responder {
    let uuid = Uuid::new_v4();
    HttpResponse::Ok().body(format!("URL: /pixel/{}", uuid))
}

#[get("/pixel/{uuid}")]
async fn pixel(req: HttpRequest, info: web::Path<Info>) -> impl Responder {
    let ip_addr = req.peer_addr().map_or_else(|| "Unknown".into(), |addr| addr.ip().to_string());
    // Utiliser ici un mécanisme d'anonymisation pour l'adresse IP avant de la logger.

    // Implémenter le logging dans un système de fichiers ou une base de données avec anonymisation de l'IP.
    println!("Anonymized IP: {}, UUID: {}", ip_addr, info.uuid);


    // Base64 encoded 1x1 transparent PNG image
    let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";

    // Decode the base64 string
    let pixel_data = decode(base64_data)
        .expect("Base64 decode error");

    HttpResponse::Ok()
        .content_type("image/png")
        .body(pixel_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurer le port via une variable d'environnement pour plus de flexibilité.
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| {
        App::new()
            .service(generate_url)
            .service(pixel)
    })
        .bind(format!("127.0.0.1:{}", server_port))?
        .run()
        .await
}
