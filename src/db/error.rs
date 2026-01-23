use std::{error::Error, fmt};
use mongodb::error::Error as MongoDriverError;

#[derive(Debug, PartialEq, Eq)]
pub enum MongoDbErrorKind {
    ConnectionFailed,
    QueryFailed,
    Timeout,
    Authentication,
    NotFound,
}

#[derive(Debug)]
pub struct MongoDbError {
    code: i32,
    message: String,
    kind: MongoDbErrorKind,
    operation: String,
    location: String,
    source: Option<MongoDriverError>,
}

impl MongoDbError {
    pub fn new(
        code: i32,
        message: &str,
        kind: MongoDbErrorKind,
        operation: &str,
        location: &str,
    ) -> Self {
        MongoDbError {
            code,
            message: message.to_string(),
            kind,
            operation: operation.to_string(),
            location: location.to_string(),
            source: None,
        }
    }

    pub fn with_source(
        code: i32,
        message: &str,
        kind: MongoDbErrorKind,
        operation: &str,
        location: &str,
        source: MongoDriverError,
    ) -> Self {
        MongoDbError {
            code,
            message: message.to_string(),
            kind,
            operation: operation.to_string(),
            location: location.to_string(),
            source: Some(source),
        }
    }

    pub fn kind(&self) -> &MongoDbErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn operation(&self) -> &str {
        &self.operation
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for MongoDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = match self.kind {
            MongoDbErrorKind::ConnectionFailed => "conexión fallida",
            MongoDbErrorKind::QueryFailed => "consulta fallida",
            MongoDbErrorKind::Timeout => "timeout",
            MongoDbErrorKind::Authentication => "autenticación fallida",
            MongoDbErrorKind::NotFound => "no encontrado",
        };
        write!(
            f,
            "[{}] MongoDB {}: {} (operación: {}, en {})",
            self.code, kind_str, self.message, self.operation, self.location
        )
    }
}

impl Error for MongoDbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e as &(dyn Error + 'static))
    }
}

impl From<MongoDriverError> for MongoDbError {
    fn from(err: MongoDriverError) -> Self {
        MongoDbError::with_source(
            2000,
            &err.to_string(),
            MongoDbErrorKind::QueryFailed,
            "unknown",
            "unknown",
            err,
        )
    }
}