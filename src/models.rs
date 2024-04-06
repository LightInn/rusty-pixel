// models.rs
use serde::{Deserialize, Serialize};
use tokio_rusqlite::{Connection as AsyncConnection};


#[derive(Serialize, Deserialize)]
pub struct Link {
    pub uuid: String,
}

// Ajoutez toutes les autres structures de données nécessaires.
pub struct AppState {
    pub db: AsyncConnection,
}


#[derive(Debug)]
pub struct PixelConnection {
    pub uuid: String,
    pub id : i32,
    pub ip: String,
    pub user_agent: String,
    pub referer: String,
    pub timestamp: String,
}

