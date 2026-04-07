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

    // Campos adicionales para información personal
    pub genero: Option<String>,
    pub nacionalidad: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub raza: Option<String>,
    pub eps: Option<String>,
    pub pension: Option<String>,
    pub cesantias: Option<String>,
    pub celular: Option<String>,
    pub correo: Option<String>,
    pub escolaridad: Option<String>,
    pub titulo_obtenido: Option<String>,
    pub departamento: Option<String>,
    pub municipio: Option<String>,
    pub tipo_vivienda: Option<String>,
    pub direccion: Option<String>,
    pub strato: Option<String>,
    pub personas_a_cargo: Option<i32>,
    pub vulnerabilidad: Option<String>,
    pub orientacion_sexual: Option<String>,
    pub pertenencia_etnica: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub contacto_emergencia_parentesco: Option<String>,
    pub tiene_vehiculo: Option<bool>,
    pub estado_civil: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct EncuestaSocioeconomicaDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genero: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacionalidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_nacimiento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raza: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cesantias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celular: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escolaridad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titulo_obtenido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departamento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_vivienda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strato: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personas_a_cargo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientacion_sexual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pertenencia_etnica: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacto_emergencia_parentesco: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiene_vehiculo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado_civil: Option<String>,
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
