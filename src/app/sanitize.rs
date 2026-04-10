/// Operadores MongoDB que no deben aparecer en valores de texto
const NOSQL_OPERATORS: &[&str] = &[
    // Comparación
    "$eq", "$gt", "$gte", "$lt", "$lte", "$ne", "$in", "$nin",
    // Lógicos
    "$or", "$and", "$not", "$nor",
    // Evaluación
    "$where", "$expr", "$regex", "$text", "$mod", "$jsonSchema",
    // Elemento
    "$exists", "$type",
    // Array
    "$all", "$elemMatch", "$size",
    // Actualización — escritura
    "$set", "$unset", "$inc", "$dec", "$mul", "$rename",
    "$push", "$pull", "$pop", "$addToSet", "$pullAll", "$each",
    "$slice", "$sort", "$bit", "$min", "$max", "$currentDate",
    // Pipeline / aggregation
    "$lookup", "$match", "$group", "$project", "$unwind", "$limit",
    "$skip", "$sort", "$count", "$bucket", "$facet", "$addFields",
    "$replaceRoot", "$replaceWith", "$merge", "$out", "$redact",
    "$geoNear", "$graphLookup", "$indexStats", "$planCacheStats",
    // Otros
    "$comment", "$natural", "$isolated", "$atomic",
];

/// Sanitiza un string que solo debe contener letras (incluye tildes, ñ), espacios y comas.
/// Retorna None si contiene cualquier otro caracter o si está vacío.
pub fn sanitize_alpha(s: String) -> Option<String> {
    let trimmed = s.trim().to_string();

    if trimmed.is_empty() || trimmed.len() > 500 {
        return None;
    }

    let valid = trimmed.chars().all(|c| {
        c.is_alphabetic() || c == ' ' || c == ','
    });

    if !valid {
        return None;
    }

    Some(trimmed)
}

/// Retorna un sanitizador que rechaza strings vacíos o que superen `max` caracteres (tras trim).
/// Diseñado para encadenarse con `and_then` antes de otros sanitizadores:
///   `dto.celular.and_then(with_max_len(15)).and_then(sanitize_numeric)`
pub fn with_max_len(max: usize) -> impl Fn(String) -> Option<String> {
    move |s: String| {
        let trimmed = s.trim().to_string();
        if trimmed.is_empty() || trimmed.len() > max {
            None
        } else {
            Some(trimmed)
        }
    }
}

/// Sanitiza un string que solo debe contener dígitos (cédulas, teléfonos).
/// Retorna None si contiene cualquier caracter no numérico o si está vacío.
pub fn sanitize_numeric(s: String) -> Option<String> {
    let trimmed = s.trim().to_string();

    if trimmed.is_empty() || !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    Some(trimmed)
}

/// Valida y sanitiza un correo electrónico.
/// Retorna None si el formato no es válido, está vacío o supera 254 caracteres.
pub fn sanitize_email(s: String) -> Option<String> {
    let trimmed = s.trim().to_lowercase();

    if trimmed.is_empty() || trimmed.len() > 254 {
        return None;
    }

    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    if parts.len() != 2 {
        return None;
    }

    let (local, domain) = (parts[0], parts[1]);

    // Local: 1-64 chars, solo alfanuméricos, puntos, guiones y guiones bajos
    if local.is_empty() || local.len() > 64 {
        return None;
    }
    if !local.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') {
        return None;
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return None;
    }

    // Dominio: debe tener al menos un punto, sin guion al inicio/fin de cada segmento
    let segments: Vec<&str> = domain.split('.').collect();
    if segments.len() < 2 {
        return None;
    }
    for seg in &segments {
        if seg.is_empty() || seg.len() > 63 {
            return None;
        }
        if !seg.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return None;
        }
        if seg.starts_with('-') || seg.ends_with('-') {
            return None;
        }
    }
    // TLD: al menos 2 caracteres, solo letras
    let tld = segments.last().unwrap();
    if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    Some(trimmed)
}

/// Valida un booleano opcional.
/// Acepta true/false directamente. Retorna None si el valor no es un booleano estricto.
/// Útil para campos como tiene_vehiculo donde no tiene sentido un valor ambiguo.
pub fn sanitize_bool(b: bool) -> Option<bool> {
    Some(b)
}

/// Parsea y valida un booleano desde string.
/// Acepta "true"/"false" (case-insensitive). Retorna None para cualquier otro valor.
pub fn sanitize_bool_str(s: String) -> Option<bool> {
    match s.trim().to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// Valida una fecha en formato YYYY-MM-DD.
/// Verifica que el formato sea correcto y que la fecha sea real (día/mes válidos, años bisiestos).
/// Retorna None si el formato es inválido o la fecha no existe.
pub fn sanitize_date(s: String) -> Option<String> {
    let trimmed = s.trim().to_string();

    if trimmed.len() != 10 {
        return None;
    }

    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let year: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;

    if year < 1900 || year > 2100 {
        return None;
    }

    if month < 1 || month > 12 {
        return None;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let bisiesto = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if bisiesto { 29 } else { 28 }
        }
        _ => return None,
    };

    if day < 1 || day > max_day {
        return None;
    }

    Some(trimmed)
}

/// Sanitiza un string contra NoSQL injection y datos inválidos.
/// Retorna None si el valor está vacío, es demasiado largo o contiene operadores MongoDB.
pub fn sanitize_text(s: String) -> Option<String> {
    let trimmed = s.trim().to_string();

    if trimmed.is_empty() || trimmed.len() > 500 {
        return None;
    }

    let lower = trimmed.to_lowercase();
    if NOSQL_OPERATORS.iter().any(|op| lower.contains(op)) {
        return None;
    }

    Some(trimmed)
}

