use mongodb::{
    Database,
    bson::Document,
    bson::doc,
    options::FindOptions,
};
use futures::TryStreamExt;

use crate::db::error::{MongoDbError, MongoDbErrorKind};
use crate::models::instalaciones::AreaFisica;

pub struct InstalacionesRepository {
    collection: mongodb::Collection<AreaFisica>,
}

pub struct QueryOptions {
    pub filter: Option<Document>,
    pub sort: Option<Document>,
    pub limit: Option<i64>,
    pub skip: Option<u64>,
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions {
            filter: None,
            sort: None,
            limit: None,
            skip: None,
        }
    }
}

impl InstalacionesRepository {
    pub fn new(db: &Database) -> Self {
        InstalacionesRepository {
            collection: db.collection("areasfisicas"),
        }
    }

    pub async fn get_data(&self, options: QueryOptions) -> Result<Vec<AreaFisica>, MongoDbError> {
        let filter = options.filter.unwrap_or_else(|| doc! {});

        let find_options = FindOptions::builder()
            .sort(options.sort)
            .limit(options.limit)
            .skip(options.skip)
            .build();

        let cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await
            .map_err(|e| {
                MongoDbError::with_source(
                    3001,
                    "Error al consultar la colección areasfisicas",
                    MongoDbErrorKind::QueryFailed,
                    "collection.find",
                    "repository/instalaciones.rs::get_data",
                    e,
                )
            })?;

        cursor.try_collect().await.map_err(|e| {
            MongoDbError::with_source(
                3002,
                "Error al leer los documentos de areasfisicas",
                MongoDbErrorKind::QueryFailed,
                "cursor.try_collect",
                "repository/instalaciones.rs::get_data",
                e,
            )
        })
    }
}
