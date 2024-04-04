use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, HttpRequest};
use std::env;
use serde::Deserialize;
use uuid::{Uuid, serde::urn};

#[derive(Deserialize)]
struct Info {
    #[serde(with = "urn")]
    uuid: Uuid,
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

    // Retourner un pixel transparent avec le type MIME approprié.
    HttpResponse::Ok()
        .content_type("image/gif")
        .body("GIF89a\\x01\\x00\\x01\\x00\\x80\\xff\\x00\\xff\\xff\\xff\\x00\\x00\\x00,\\x00\\x00\\x00\\x00\\x01\\x00\\x01\\x00\\x00\\x02\\x02L\\x01\\x00;")
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
