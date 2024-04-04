// db.rs
use rusqlite::{params, Connection, Result};

pub fn init(database_url: &str) -> Result<Connection> {
    let conn = Connection::open(database_url)?;

    // Initialiser les tables, etc.
    create_tables(&conn)?;
    // Initialiser les index, etc.

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pixel (
            uuid TEXT PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pixel_connection (
            id INTEGER PRIMARY KEY,
            uuid TEXT NOT NULL,
            ip TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            user_agent TEXT
        )",
        params![],
    )?;
    Ok(())
}