use crate::db::mongodb::MongoDb;

pub struct HealthStatus {
    pub db_ok: bool,
    pub db_message: String,
}

pub async fn check_health(db: &MongoDb) -> HealthStatus {
    let result = db
        .db
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await;

    match result {
        Ok(_) => HealthStatus {
            db_ok: true,
            db_message: "MongoDB conectado".to_string(),
        },
        Err(e) => HealthStatus {
            db_ok: false,
            db_message: format!("MongoDB no disponible: {}", e),
        },
    }
}
