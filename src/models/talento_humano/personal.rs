use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize, Serializer};

use crate::models::talento_humano::cargos_personal::CargosPersonal;
use crate::models::utils::serialize_oid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid"
    )]
    pub id: Option<ObjectId>,
    #[serde(rename = "PE")]
    pub pe: i64,
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo: Option<CargoRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identificacion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_documento: Option<TipoDocumento>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_sangre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_identificacion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_foto_carnet: Option<String>,
    pub estado: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_oid"
    )]
    pub carnet: Option<ObjectId>,
    #[serde(rename = "__v", skip_serializing)]
    pub v: Option<i32>,

    // Campos adicionales para información personal
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
    #[serde(rename = "fecha_formulario_sociodemografico", skip_serializing_if = "Option::is_none")]
    pub fecha_formulario_sociodemografico: Option<DateTime>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_formulario_sociodemografico: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TipoDocumento {
    #[serde(rename = "cedula", alias = "Cedula")]
    Cedula,
    #[serde(rename = "pasaporte", alias = "Pasaporte")]
    Pasaporte,
    #[serde(
        rename = "cedula_extranjera",
        alias = "Cedula_extranjera",
        alias = "CedulaExtranjera"
    )]
    CedulaExtranjera,
}
