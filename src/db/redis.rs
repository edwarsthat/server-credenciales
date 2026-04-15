use redis::{Client, aio::ConnectionManager};

#[derive(Clone)]
pub struct RedisDb {
    pub manager: ConnectionManager,
}

impl RedisDb {
    pub async fn create_connection(uri: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(uri)?;
        let manager = ConnectionManager::new(client).await?;

        println!("Conexión a Redis establecida");

        Ok(Self { manager })
    }
}
