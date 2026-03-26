use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::talento_humano::cargos_personal::CargosPersonal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "PE")]
    pub pe: i64,
    pub nombre: String,
    pub cargo: Option<CargoRef>,
    pub identificacion: String,
    pub tipo_documento: TipoDocumento,
    pub foto: Option<String>,
    pub tipo_sangre: Option<String>,
    pub url_identificacion: String,
    pub url_foto_carnet: String,
    pub estado: bool,
    pub carnet: Option<ObjectId>,
    #[serde(rename = "__v", skip_serializing)]
    pub v: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CargoRef {
    Id(ObjectId),
    Populated(Box<CargosPersonal>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TipoDocumento {
    #[serde(rename = "cedula")]
    Cedula,
    #[serde(rename = "pasaporte")]
    Pasaporte,
    #[serde(rename = "cedula_extranjera")]
    CedulaExtranjera,
}
