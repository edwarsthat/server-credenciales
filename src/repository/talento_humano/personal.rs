use mongodb::{
    Database,
    bson::{Document, doc},
    options::FindOptions,
};
use futures::TryStreamExt;

use crate::db::error::{MongoDbError, MongoDbErrorKind};
use crate::models::talento_humano::personal::Personal;

pub struct PersonalRepository {
    collection: mongodb::Collection<Personal>,
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

impl PersonalRepository {
    pub fn new(db: &Database) -> Self {
        PersonalRepository {
            collection: db.collection("personals"),
        }
    }

    pub async fn get_data(&self, options: QueryOptions) -> Result<Vec<Personal>, MongoDbError> {
        if options.populate {
            self.get_data_populated(options).await
        } else {
            self.get_data_plain(options).await
        }
    }

    async fn get_data_plain(&self, options: QueryOptions) -> Result<Vec<Personal>, MongoDbError> {
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
                    2003,
                    "Error al consultar la colección personal",
                    MongoDbErrorKind::QueryFailed,
                    "collection.find",
                    "repository/talento_humano/personal.rs::get_data_plain",
                    e,
                )
            })?;

        cursor.try_collect().await.map_err(|e| {
            MongoDbError::with_source(
                2004,
                "Error al leer los documentos de personal",
                MongoDbErrorKind::QueryFailed,
                "cursor.try_collect",
                "repository/talento_humano/personal.rs::get_data_plain",
                e,
            )
        })
    }

    async fn get_data_populated(&self, options: QueryOptions) -> Result<Vec<Personal>, MongoDbError> {
        let mut pipeline: Vec<Document> = Vec::new();

        if let Some(filter) = options.filter {
            pipeline.push(doc! { "$match": filter });
        }

        pipeline.push(doc! {
            "$lookup": {
                "from": "cargospersonals",
                "localField": "cargo",
                "foreignField": "_id",
                "as": "cargo"
            }
        });

        pipeline.push(doc! {
            "$unwind": {
                "path": "$cargo",
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
                    2010,
                    "Error al ejecutar aggregate en personal",
                    MongoDbErrorKind::QueryFailed,
                    "collection.aggregate",
                    "repository/talento_humano/personal.rs::get_data_populated",
                    e,
                )
            })?;

        let docs: Vec<Document> = cursor.try_collect().await.map_err(|e| {
            MongoDbError::with_source(
                2011,
                "Error al leer documentos del aggregate de personal",
                MongoDbErrorKind::QueryFailed,
                "cursor.try_collect",
                "repository/talento_humano/personal.rs::get_data_populated",
                e,
            )
        })?;

        println!(">>> personal docs encontrados: {}", docs.len());
        for d in &docs {
            println!(">>> doc: {:?}", d);
        }

        docs.into_iter()
            .map(|doc| {
                mongodb::bson::from_document(doc).map_err(|e| {
                    MongoDbError::new(
                        2012,
                        &format!("Error deserializando personal: {}", e),
                        MongoDbErrorKind::QueryFailed,
                        "bson::from_document",
                        "repository/talento_humano/personal.rs::get_data_populated",
                    )
                })
            })
            .collect()
    }
}
