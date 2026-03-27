use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::utils::serialize_oid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaFisica {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub id: Option<ObjectId>,
    pub nombre: String,
    pub sede: String,
}
