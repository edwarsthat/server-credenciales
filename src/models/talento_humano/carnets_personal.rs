use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::models::utils::serialize_oid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarnetPersonal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub tipo: CarnetType,
    pub status: CarnetStatus,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub employee_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_hash: Option<String>,
    pub is_generated: bool,
    #[serde(rename = "SKU")]
    pub sku: i64,
    pub issued_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub user: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub assigned_by: Option<ObjectId>,
    pub notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CarnetType {
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "final")]
    Final,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CarnetStatus {
    #[serde(rename = "stock")]
    Stock,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "expired")]
    Expired,
}
