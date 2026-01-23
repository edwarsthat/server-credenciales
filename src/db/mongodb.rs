use mongodb::{Client, Database};

use crate::db::error::{MongoDbError, MongoDbErrorKind};

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub db: Database,
}

impl MongoDb {
    pub async fn create_connection(uri: &str, db_name: &str) -> Result<Self, MongoDbError> {
        let client = Client::with_uri_str(uri).await.map_err(|e| {
            MongoDbError::with_source(
                2001,
                "No se pudo conectar a MongoDB",
                MongoDbErrorKind::ConnectionFailed,
                "Client::with_uri_str",
                "mongodb.rs::create_connection",
                e,
            )
        })?;

        let db = client.database(db_name);

        // Verificar conexión con un ping
        db.run_command(mongodb::bson::doc! { "ping": 1 })
            .await
            .map_err(|e| {
                MongoDbError::with_source(
                    2002,
                    "No se pudo verificar la conexión (ping fallido)",
                    MongoDbErrorKind::ConnectionFailed,
                    "db.run_command(ping)",
                    "mongodb.rs::create_connection",
                    e,
                )
            })?;

        println!("Conexión a MongoDB establecida: {}", db_name);

        Ok(Self { client, db })
    }
}