use mongodb::{
    Database,
    bson::{Document, doc},
    options::FindOptions,
};
use futures::TryStreamExt;

use crate::db::error::{MongoDbError, MongoDbErrorKind};
use crate::models::talento_humano::carnets_personal::CarnetPersonal;

pub struct CarnetsRepository {
    collection: mongodb::Collection<CarnetPersonal>,
}

pub struct QueryOptions {
    pub filter: Option<Document>,
    pub sort: Option<Document>,
    pub limit: Option<i64>,
    pub skip: Option<u64>,
    pub populate: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions {
            filter: None,
            sort: None,
            limit: None,
            skip: None,
            populate: false,
        }
    }
}

impl CarnetsRepository {
    pub fn new(db: &Database) -> Self {
        CarnetsRepository {
            collection: db.collection("carnets"),
        }
    }

    pub async fn get_data(&self, options: QueryOptions) -> Result<Vec<CarnetPersonal>, MongoDbError> {
        if options.populate {
            self.get_data_populated(options).await
        } else {
            self.get_data_plain(options).await
        }
    }

    async fn get_data_plain(&self, options: QueryOptions) -> Result<Vec<CarnetPersonal>, MongoDbError> {
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
                    2005,
                    "Error al consultar la colección carnet",
                    MongoDbErrorKind::QueryFailed,
                    "collection.find",
                    "repository/talento_humano/carnets.rs::get_data_plain",
                    e,
                )
            })?;

        cursor.try_collect().await.map_err(|e| {
            MongoDbError::with_source(
                2006,
                "Error al leer los documentos de carnet",
                MongoDbErrorKind::QueryFailed,
                "cursor.try_collect",
                "repository/talento_humano/carnets.rs::get_data_plain",
                e,
            )
        })
    }

    async fn get_data_populated(&self, options: QueryOptions) -> Result<Vec<CarnetPersonal>, MongoDbError> {
        let mut pipeline: Vec<Document> = Vec::new();

        // Filtro
        if let Some(filter) = options.filter {
            pipeline.push(doc! { "$match": filter });
        }

        // Lookup: employee_id → colección personal
        pipeline.push(doc! {
            "$lookup": {
                "from": "personal",
                "localField": "employeeId",
                "foreignField": "_id",
                "as": "employeeId"
            }
        });

        // $lookup devuelve array, con $unwind lo convertimos a objeto (preservando null)
        pipeline.push(doc! {
            "$unwind": {
                "path": "$employeeId",
                "preserveNullAndEmptyArrays": true
            }
        });

        if let Some(sort) = options.sort {
            pipeline.push(doc! { "$sort": sort });
        }
        if let Some(skip) = options.skip {
            pipeline.push(doc! { "$skip": skip as i64 });
        }
        if let Some(limit) = options.limit {
            pipeline.push(doc! { "$limit": limit });
        }

        let cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(|e| {
                MongoDbError::with_source(
                    2007,
                    "Error al ejecutar aggregate en carnet",
                    MongoDbErrorKind::QueryFailed,
                    "collection.aggregate",
                    "repository/talento_humano/carnets.rs::get_data_populated",
                    e,
                )
            })?;

        let docs: Vec<Document> = cursor.try_collect().await.map_err(|e| {
            MongoDbError::with_source(
                2008,
                "Error al leer documentos del aggregate",
                MongoDbErrorKind::QueryFailed,
                "cursor.try_collect",
                "repository/talento_humano/carnets.rs::get_data_populated",
                e,
            )
        })?;

        docs.into_iter()
            .map(|doc| {
                mongodb::bson::from_document(doc).map_err(|e| {
                    MongoDbError::new(
                        2009,
                        &format!("Error deserializando carnet: {}", e),
                        MongoDbErrorKind::QueryFailed,
                        "bson::from_document",
                        "repository/talento_humano/carnets.rs::get_data_populated",
                    )
                })
            })
            .collect()
    }
}
