use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::cookie::time::error::Format::StdIo;
use base64::decode;
use uuid::Uuid;
use web::Data;
use crate::db;


use crate::models::{AppState, Link};

#[get("/generate")]
pub async fn generate(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;


    let uuid = Uuid::new_v4();
    // uuid to string
    let uuidStr = uuid.to_string();


    // Ici, vous pourriez insérer le nouveau UUID dans la base de données avant de le retourner
    // à l'utilisateur en JSON

    db::insert_pixel(db, uuidStr).await.expect("Failed to insert pixel");


    HttpResponse::Ok().json(Link { uuid: (&uuid).to_string(), url: format!("/pixel/{}", (&uuid).to_string()) })
}


#[get("/pixel/{uuid}")]
pub async fn track_pixel(req: HttpRequest, info: web::Path<Link>, data: web::Data<AppState>) -> impl Responder {
    let ip_addr = req
        .peer_addr()
        .map_or_else(|| "Unknown".into(), |addr| addr.ip().to_string());
    // Utiliser ici un mécanisme d'anonymisation pour l'adresse IP avant de la logger.

    // Implémenter le logging dans un système de fichiers ou une base de données avec anonymisation de l'IP.
    println!("Anonymized IP: {}, UUID: {}", ip_addr, info.uuid);


    let user_agent: String = req
        .headers()
        .get("User-Agent")
        .map_or_else(|| "Unknown".into(), |ua| ua.to_str().unwrap().into());

    println!("User-Agent: {}", user_agent);


    // Ici, vous pourriez insérer les informations de connexion dans la base de données.
    db::insert_pixel_connection(&data.db, &*info.uuid, &*ip_addr, &*user_agent).await.expect("Failed to insert pixel connection");


    // Base64 encoded 1x1 transparent PNG image
    let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";

    // Decode the base64 string
    let pixel_data = decode(base64_data).expect("Base64 decode error");

    HttpResponse::Ok()
        .content_type("image/png")
        .body(pixel_data)
}


// create a page to list all the pixels in the database
#[get("/pixels")]
pub async fn list_pixels(data: Data<AppState>) -> impl Responder {
    let db = &data.db;

    let mut response = String::from("<html><head><title>Pixel List</title></head><body><h1>Pixel List</h1><ul>");

    let all_pixel = db::fetch_all_pixels(db).await.expect("Failed to fetch pixels");

    // sort the pixels by timestamp
    let mut all_pixel = all_pixel;
    all_pixel.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));


    for pixel in all_pixel {


        // link to view all connections for a specific pixel
        response.push_str(&format!(
            "<li> Date : {}  <a href=\"/pixel/connections/{}\">  uuid : {} </a></li>",
            pixel.timestamp, pixel.uuid, pixel.uuid
        ));
    }

    response.push_str("</ul>");
    // buton to generate a new pixel
    response.push_str("<a href=\"/generate\">Generate a new pixel</a></body></html>");

    HttpResponse::Ok().body(response)
}


// create a page to list all the pixel connections from a specific pixel
#[get("/pixel/connections/{uuid}")]
pub async fn list_pixel_connections(data: Data<AppState>, info: web::Path<Link>) -> impl Responder {
    let db = &data.db;

    println!("UUID: {}", info.uuid);


    let mut response = String::from("<html><head><title>Pixel Connections</title></head><body><h1>Pixel Connections</h1><ul>");

    let all_pixel_connections = db::fetch_all_pixel_connections(db, &*info.uuid).await.expect("Failed to fetch pixel connections");

    for connection in all_pixel_connections {
        response.push_str(&format!("<li>IP: {}, User-Agent: {}, Timestamp: {}</li>", connection.ip, connection.user_agent, connection.timestamp));
        // response.push_str(&format!("<li>Connection: {}</li>", connection));
    }

    response.push_str("</ul></body></html>");

    HttpResponse::Ok().body(response)
}



