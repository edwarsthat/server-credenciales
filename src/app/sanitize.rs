/// Operadores MongoDB que no deben aparecer en valores de texto
const NOSQL_OPERATORS: &[&str] = &[
    "$where", "$gt", "$gte", "$lt", "$lte", "$ne", "$in", "$nin",
    "$or", "$and", "$not", "$nor", "$exists", "$type", "$expr",
    "$regex", "$text", "$mod", "$all", "$elemMatch", "$size",
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

/// Sanitiza un string que solo debe contener dígitos (cédulas, teléfonos).
/// Retorna None si contiene cualquier caracter no numérico o si está vacío.
pub fn sanitize_numeric(s: String) -> Option<String> {
    let trimmed = s.trim().to_string();

    if trimmed.is_empty() || !trimmed.chars().all(|c| c.is_ascii_digit()) {
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
