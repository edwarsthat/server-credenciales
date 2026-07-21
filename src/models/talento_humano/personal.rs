use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize, Serializer};

use crate::models::talento_humano::cargos_personal::CargosPersonal;
use crate::models::utils::{serialize_oid, serialize_datetime};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo: Option<CargoRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foto: Option<String>,
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
    pub nombre: String,
    pub apellido: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identificacion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_documento: Option<TipoDocumento>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tipo_sangre")]
    pub tipo_sangre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genero: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacionalidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "fecha_nacimiento")]
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
    pub estrato: Option<String>,
    #[serde(rename = "personasACargo", skip_serializing_if = "Option::is_none")]
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
    #[serde(
        rename = "fecha_formulario_sociodemografico",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime"
    )]
    pub fecha_formulario_sociodemografico: Option<DateTime>,

    // Informacion familiar
    #[serde(rename = "nombre_conyugue", skip_serializing_if = "Option::is_none")]
    pub nombre_conyugue: Option<String>,
    #[serde(rename = "apellido_conyugue", skip_serializing_if = "Option::is_none")]
    pub apellido_conyugue: Option<String>,
    #[serde(rename = "telefono_conyugue", skip_serializing_if = "Option::is_none")]
    pub telefono_conyugue: Option<String>,
    #[serde(rename = "tiempo_conviviendo", skip_serializing_if = "Option::is_none")]
    pub tiempo_conviviendo: Option<i32>,
    #[serde(rename = "tiene_hijos", skip_serializing_if = "Option::is_none")]
    pub tiene_hijos: Option<bool>,
    #[serde(rename = "cuantos_hijos", skip_serializing_if = "Option::is_none")]
    pub cuantos_hijos: Option<i32>,
    #[serde(rename = "edad_hijos", skip_serializing_if = "Option::is_none")]
    pub edad_hijos: Option<Vec<i32>>,

    // Dotacion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camisa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pantalon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calzado: Option<String>,
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
    pub nombre: String,
    pub apellido: String,
    pub identificacion: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tipo_documento")]
    pub tipo_documento: Option<TipoDocumento>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tipo_sangre")]
    pub tipo_sangre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genero: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacionalidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "fecha_nacimiento")]
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
    #[serde(skip_serializing_if = "Option::is_none", alias = "titulo_obtenido")]
    pub titulo_obtenido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departamento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tipo_vivienda")]
    pub tipo_vivienda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estrato: Option<String>,
    #[serde(rename = "personasACargo", skip_serializing_if = "Option::is_none", alias = "personas_a_cargo")]
    pub personas_a_cargo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "orientacion_sexual")]
    pub orientacion_sexual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "pertenencia_etnica")]
    pub pertenencia_etnica: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contacto_emergencia_nombre")]
    pub contacto_emergencia_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contacto_emergencia_telefono")]
    pub contacto_emergencia_telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contacto_emergencia_parentesco")]
    pub contacto_emergencia_parentesco: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tiene_vehiculo")]
    pub tiene_vehiculo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "estado_civil")]
    pub estado_civil: Option<String>,
    #[serde(
        rename = "fecha_formulario_sociodemografico",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_datetime",
        alias = "fecha_formulario_sociodemografico"
    )]
    pub fecha_formulario_sociodemografico: Option<DateTime>,

    // Informacion familiar
    #[serde(rename = "nombre_conyugue", skip_serializing_if = "Option::is_none")]
    pub nombre_conyugue: Option<String>,
    #[serde(rename = "apellido_conyugue", skip_serializing_if = "Option::is_none")]
    pub apellido_conyugue: Option<String>,
    #[serde(rename = "telefono_conyugue", skip_serializing_if = "Option::is_none")]
    pub telefono_conyugue: Option<String>,
    #[serde(rename = "tiempo_conviviendo", skip_serializing_if = "Option::is_none")]
    pub tiempo_conviviendo: Option<i32>,
    #[serde(rename = "tiene_hijos", skip_serializing_if = "Option::is_none")]
    pub tiene_hijos: Option<bool>,
    #[serde(rename = "cuantos_hijos", skip_serializing_if = "Option::is_none")]
    pub cuantos_hijos: Option<i32>,
    #[serde(rename = "edad_hijos", skip_serializing_if = "Option::is_none")]
    pub edad_hijos: Option<Vec<i32>>,

    // Dotacion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camisa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pantalon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calzado: Option<String>,
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
