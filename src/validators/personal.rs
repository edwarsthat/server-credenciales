use crate::app::error::ApiError;
use crate::app::sanitize::{sanitize_alpha, sanitize_bool, sanitize_date, sanitize_email, sanitize_numeric, sanitize_text, with_max_len};
use crate::models::talento_humano::personal::EncuestaSocioeconomicaDto;

pub fn validate_encuesta(dto: EncuestaSocioeconomicaDto) -> Result<EncuestaSocioeconomicaDto, ApiError> {
    let nombre = sanitize_alpha(dto.nombre)
        .ok_or_else(|| ApiError::BadRequest("Nombre inválido".to_string()))?;

    let apellido = sanitize_alpha(dto.apellido)
        .ok_or_else(|| ApiError::BadRequest("Apellido inválido".to_string()))?;

    let identificacion = sanitize_text(dto.identificacion)
        .and_then(|s| if s.len() <= 20 { Some(s) } else { None })
        .ok_or_else(|| ApiError::BadRequest("Identificación inválida".to_string()))?;

    Ok(EncuestaSocioeconomicaDto {
        nombre,
        apellido,
        identificacion,
        tipo_documento: dto.tipo_documento,
        tipo_sangre: dto.tipo_sangre.and_then(|s| with_max_len(3)(s)).and_then(sanitize_text),
        genero: dto.genero.and_then(sanitize_alpha),
        nacionalidad: dto.nacionalidad.and_then(sanitize_text),
        fecha_nacimiento: dto.fecha_nacimiento.and_then(sanitize_date),
        raza: dto.raza.and_then(sanitize_alpha),
        eps: dto.eps.and_then(sanitize_alpha),
        pension: dto.pension.and_then(sanitize_alpha),
        cesantias: dto.cesantias.and_then(sanitize_alpha),
        celular: dto.celular.and_then(with_max_len(15)).and_then(sanitize_numeric),
        correo: dto.correo.and_then(sanitize_email),
        escolaridad: dto.escolaridad.and_then(sanitize_alpha),
        titulo_obtenido: dto.titulo_obtenido.and_then(sanitize_alpha),
        departamento: dto.departamento.and_then(sanitize_alpha),
        municipio: dto.municipio.and_then(sanitize_alpha),
        tipo_vivienda: dto.tipo_vivienda.and_then(sanitize_alpha),
        direccion: dto.direccion.and_then(sanitize_text),
        strato: dto.strato.and_then(with_max_len(2)).and_then(sanitize_numeric),
        vulnerabilidad: dto.vulnerabilidad.and_then(sanitize_alpha),
        orientacion_sexual: dto.orientacion_sexual.and_then(sanitize_alpha),
        pertenencia_etnica: dto.pertenencia_etnica.and_then(sanitize_alpha),
        contacto_emergencia_nombre: dto.contacto_emergencia_nombre.and_then(sanitize_alpha),
        contacto_emergencia_telefono: dto.contacto_emergencia_telefono.and_then(with_max_len(15)).and_then(sanitize_numeric),
        contacto_emergencia_parentesco: dto.contacto_emergencia_parentesco.and_then(sanitize_alpha),
        tiene_vehiculo: dto.tiene_vehiculo.and_then(sanitize_bool),
        estado_civil: dto.estado_civil.and_then(sanitize_alpha),
        fecha_formulario_sociodemografico: dto.fecha_formulario_sociodemografico.and_then(sanitize_date),
        personas_a_cargo: dto.personas_a_cargo.filter(|&n| n >= 0 && n <= 20),
    })
}
