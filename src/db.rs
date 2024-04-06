// db.rs
use rusqlite::{params, Result};
use tokio_rusqlite::{Connection as AsyncConnection, Connection};
use crate::models::PixelConnection;

/// Initializes the database by opening a connection to the given database URL and
/// creating necessary tables if they do not exist.
///
/// # Arguments
///
/// * `database_url` - A string slice that holds the database URL.
///
/// # Returns
///
/// A result containing the database connection object or an error.
pub async fn init(database_url: &str) -> Result<AsyncConnection> {
    let conn = AsyncConnection::open(database_url).await.expect("Failed to open database");

    // Initialize tables and indexes.
    create_tables(&conn).await.expect("TODO: panic message");

    Ok(conn)
}

/// Creates tables within the database to store pixel generation and connection data.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection object.
///
/// # Returns
///
/// A result indicating success or containing an error.
async fn create_tables(conn: &AsyncConnection) -> Result<()> {
    conn.call_unwrap(|conn| {


        // Table to store each generated pixel with the creation timestamp
        // to avoid duplicates and remember when it was generated.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pixel (
                uuid TEXT PRIMARY KEY,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).unwrap();


        // Table to store each connection to the pixel with associated
        // IP address, user agent, timestamp, etc.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pixel_connection (
            id INTEGER PRIMARY KEY,
            uuid TEXT NOT NULL,
            ip TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            user_agent TEXT
        )",
            [],
        ).unwrap();
    }).await;


    Ok(())
}

/// Inserts a new pixel into the database with its UUID and creation timestamp.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection object.
/// * `uuid` - The UUID of the pixel to insert.
///
/// # Returns
///
/// A result indicating success or containing an error.
pub async fn insert_pixel(conn: &AsyncConnection, uuid: String) -> Result<()> {
    let uuid = uuid.to_string();


    conn.call_unwrap(move |conn| {
        conn.execute(
            "INSERT INTO pixel (uuid) VALUES (?1)",
            params![uuid],
        ).unwrap();
    }).await;

    Ok(())
}

/// Inserts a new pixel connection into the database with IP address,
/// UUID of the pixel, timestamp, and user agent.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection object.
/// * `uuid` - The UUID of the pixel being accessed.
/// * `ip` - The IP address of the client making the request.
/// * `user_agent` - The user agent string of the client.
///
/// # Returns
///
/// A result indicating success or containing an error.
pub async fn insert_pixel_connection(
    conn: &AsyncConnection,
    uuid: &str,
    ip: &str,
    user_agent: &str,
) -> Result<()> {
    let uuid = uuid.to_string();
    let ip = ip.to_string();
    let user_agent = user_agent.to_string();


    conn.call_unwrap(move |conn| {
        conn.execute(
            "INSERT INTO pixel_connection (uuid, ip, user_agent) VALUES (?1, ?2, ?3)",
            params![uuid, ip, user_agent],
        ).unwrap();
    }).await;


    Ok(())
}


// fetch all pixels from the database
pub async fn fetch_all_pixels(conn: &AsyncConnection) -> Result<Vec<String>> {
    let pixel = conn.call_unwrap(move |conn| {
        let mut stmt = conn.prepare("SELECT uuid, timestamp FROM pixel").unwrap();
        let pixel_iter = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        }).unwrap();


        let mut pixels = Vec::new();
        for pixel in pixel_iter {
            pixels.push(pixel.unwrap());
        }

        pixels
    }).await;

    println!("{:?}", pixel);
    Ok(pixel)
}


// fetch all pixel connections from the database
pub async fn fetch_all_pixel_connections(conn: &AsyncConnection, uuid: &str) -> Result<Vec<PixelConnection>> {



    let uuid = uuid.to_string();



    let pixel_connections = conn.call_unwrap(move |conn| {
        let mut stmt = conn.prepare("SELECT id, ip, timestamp, user_agent FROM pixel_connection WHERE uuid = :uuid").unwrap();


        let pixel_connection_iter = stmt.query_map(&[(":uuid", &uuid)], |row| {
            Ok(PixelConnection {
                uuid: uuid.to_string(),
                id : row.get(0)?,
                ip: row.get(1)?,
                timestamp: row.get(2)?,
                user_agent: row.get(3)?,
                referer: "".to_string(),
            })
        }).unwrap();


        let mut pixel_connection = Vec::new();
        for pixel_con in pixel_connection_iter {
            pixel_connection.push(pixel_con.unwrap()  );
        }


        pixel_connection
    }).await;


    println!(" Get data {:?}", pixel_connections);

    Ok(pixel_connections)
}