use std::{error::Error, fmt};


#[derive(Debug, PartialEq, Eq)]
pub enum EnvVarErrorKind {
    Missing,
    Invalid,
}

#[derive(Debug)]
pub struct EnvVarError {
    code: i32,
    message: String,
    kind: EnvVarErrorKind,
    var_name: String,
    location: String,
}

impl EnvVarError {
    pub fn new(code: i32, message: &str, kind: EnvVarErrorKind, var_name: &str, location: &str) -> Self {
        EnvVarError {
            code,
            message: message.to_string(),
            kind,
            var_name: var_name.to_string(),
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &EnvVarErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn var_name(&self) -> &str {
        &self.var_name
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for EnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "[{}] Variable de entorno '{}' {}: {} (en {})",
            self.code,
            self.var_name,
            match self.kind {
                EnvVarErrorKind::Missing => "faltante",
                EnvVarErrorKind::Invalid => "inválida",
            },
            self.message,
            self.location
        )
    }
}

impl Error for EnvVarError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}