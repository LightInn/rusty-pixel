use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use base64::decode;
use uuid::Uuid;
use web::Data;
use crate::models::{AppState, Info};

#[get("/generate-url")]
pub async fn generate_url() -> impl Responder {
    let uuid = Uuid::new_v4();
    // Ici, vous pourriez insérer le nouveau UUID dans la base de données avant de le retourner
    HttpResponse::Ok().body(format!("URL: /pixel/{}", uuid))
}

#[get("/pixel/{uuid}")]
pub async fn pixel(req: HttpRequest, info: web::Path<Info>) -> impl Responder {
    let ip_addr = req
        .peer_addr()
        .map_or_else(|| "Unknown".into(), |addr| addr.ip().to_string());
    // Utiliser ici un mécanisme d'anonymisation pour l'adresse IP avant de la logger.

    // Implémenter le logging dans un système de fichiers ou une base de données avec anonymisation de l'IP.
    println!("Anonymized IP: {}, UUID: {}", ip_addr, info.uuid);

    // Base64 encoded 1x1 transparent PNG image
    let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";

    // Decode the base64 string
    let pixel_data = decode(base64_data).expect("Base64 decode error");

    HttpResponse::Ok()
        .content_type("image/png")
        .body(pixel_data)
}
