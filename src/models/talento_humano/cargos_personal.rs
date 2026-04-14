use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::utils::{serialize_oid, serialize_oid_vec};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargosPersonal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub id: Option<ObjectId>,
    pub nombre: String,
    #[serde(rename = "areasAcceso", serialize_with = "serialize_oid_vec")]
    pub areas: Vec<ObjectId>,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "#7EBA27")]
    Verde,
    #[serde(rename = "#FFCD00")]
    Amarillo,
    #[serde(rename = "#F3930D")]
    Naranja,
}
