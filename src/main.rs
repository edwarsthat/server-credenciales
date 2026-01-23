use std::error::Error;
use std::process;
use tokio::net::TcpListener;
use credenciales_service::config::env;
use credenciales_service::{app, db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        if let Some(source) = e.source() {
            eprintln!("Caused by:{source}")
        }
        process::exit(1);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = env::load_config()?;
    let mongo_client =
        db::mongodb::MongoDb::create_connection(&config.mongo_url(), &config.db_name).await?;

    let addr = format!("{}:{}", config.host, config.port);
    let app = app::app::create_router(mongo_client);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("Servidor escuchando en {}", addr);
            listener
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    axum::serve(listener, app).await?;

    Ok(())
}
