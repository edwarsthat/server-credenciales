use mongodb::bson::{DateTime, doc, oid::ObjectId, to_document};

use crate::{
    app::{error::ApiError, extractor::TokenData},
    db::mongodb::MongoDb,
    models::talento_humano::personal::{EncuestaSocioeconomicaDto, Personal},
    repository::talento_humano::personal::{PersonalRepository, QueryOptions, UpdateOptions},
    validators::personal::validate_encuesta,
};

pub async fn encuesta_socioeconomica_controller(
    db: &MongoDb,
    empleado_id: ObjectId,
    body: EncuestaSocioeconomicaDto,
) -> Result<(), ApiError> {
    let body = validate_encuesta(body)?;

    let mut set = to_document(&body)
        .map_err(|e| ApiError::InternalError(format!("Error al serializar datos: {}", e)))?;

    if set.is_empty() {
        return Err(ApiError::BadRequest(
            "No se enviaron campos para actualizar".to_string(),
        ));
    }

    set.insert("fecha_formulario_sociodemografico", DateTime::now());

    let repo = PersonalRepository::new(&db.db);

    repo.patch_data(UpdateOptions {
        filter: Some(doc! { "_id": empleado_id }),
        update: Some(doc! { "$set": set }),
    })
    .await
    .map_err(|e| ApiError::InternalError(e.message().to_string()))
}

pub async fn get_encuesta_socioeconomica_controller(
    db: &MongoDb,
    token_data: TokenData,
) -> Result<Personal, ApiError> {
    let id = token_data.empleado_id;
    let personal_repo = PersonalRepository::new(&db.db);

    let mut results = personal_repo
        .get_data(QueryOptions {
            filter: Some(doc! { "_id": id }),
            projection: Some(doc! {
                "_id": 1,
                "PE": 1,
                "nombre": 1,
                "apellido": 1,
                "cargo": 1,
                "identificacion": 1,
                "tipoDocumento": 1,
                "foto": 1,
                "tipoSangre": 1,
                "urlFotoCarnet": 1,
                "estado": 1,
                "fecha_formulario_sociodemografico": 1,
                "genero": 1,
                "nacionalidad": 1,
                "fechaNacimiento": 1,
                "raza": 1,
                "eps": 1,
                "pension": 1,
                "cesantias": 1,
                "celular": 1,
                "correo": 1,
                "escolaridad": 1,
                "tituloObtenido": 1,
                "departamento": 1,
                "municipio": 1,
                "tipoVivienda": 1,
                "direccion": 1,
                "strato": 1,
                "personas_a_cargo": 1,
                "vulnerabilidad": 1,
                "orientacionSexual": 1,
                "pertenenciaEtnica": 1,
                "contactoEmergenciaNombre": 1,
                "contactoEmergenciaTelefono": 1,
                "contactoEmergenciaParentesco": 1,
                "tieneVehiculo": 1,
                "estadoCivil": 1
            }),
            ..Default::default()
        })
        .await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))?;

    results
        .pop()
        .ok_or_else(|| ApiError::NotFound("Empleado no encontrado".to_string()))
}
