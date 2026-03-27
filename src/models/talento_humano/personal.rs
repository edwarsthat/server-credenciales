use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

use crate::models::talento_humano::cargos_personal::CargosPersonal;
use crate::models::utils::serialize_oid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
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
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_oid")]
    pub carnet: Option<ObjectId>,
    #[serde(rename = "__v", skip_serializing)]
    pub v: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CargoRef {
    Id(ObjectId),
    Populated(Box<CargosPersonal>),
}

impl Serialize for CargoRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CargoRef::Id(oid) => serializer.serialize_str(&oid.to_hex()),
            CargoRef::Populated(cargo) => cargo.serialize(serializer),
        }
    }
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
