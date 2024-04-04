// models.rs
use serde::Deserialize;
use tokio_rusqlite::{Connection as AsyncConnection};


#[derive(Deserialize)]
pub struct Info {
    pub uuid: String,
}

// Ajoutez toutes les autres structures de données nécessaires.
pub struct AppState {
    pub db: AsyncConnection,
}
