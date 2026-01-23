use crate::config::error::{EnvVarError, EnvVarErrorKind};
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_replica_set: String,
    pub db_auth_source: String,
}

impl ServerConfig {
    /// Construye la URL de conexión a MongoDB
    pub fn mongo_url(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/?replicaSet={}&authSource={}",
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_replica_set,
            self.db_auth_source
        )
    }
}

/// Carga la configuración desde el archivo .env y variables de entorno
pub fn load_config() -> Result<ServerConfig, EnvVarError> {
    dotenv().ok();
    load_config_from_env()
}

/// Lee la configuración directamente de las variables de entorno (sin cargar .env)
fn load_config_from_env() -> Result<ServerConfig, EnvVarError> {
    let host = get_env_string("HOST")?;
    let port = get_env_port("PORT")?;
    let db_user = get_env_string("DB_USER")?;
    let db_password = get_env_string("DB_PASSWORD")?;
    let db_host = get_env_string("DB_HOST")?;
    let db_port = get_env_port("DB_PORT")?;
    let db_name = get_env_string("DB_NAME")?;
    let db_replica_set = get_env_string("DB_REPLICA_SET")?;
    let db_auth_source = get_env_string("DB_AUTH_SOURCE")?;

    println!(
        "Configuración cargada: HOST={}, PORT={}, DB_HOST={}, DB_PORT={}",
        host, port, db_host, db_port
    );

    Ok(ServerConfig {
        host,
        port,
        db_user,
        db_password,
        db_host,
        db_port,
        db_name,
        db_replica_set,
        db_auth_source,
    })
}

fn get_env_string(var_name: &str) -> Result<String, EnvVarError> {
    match env::var(var_name) {
        Ok(val) if !val.trim().is_empty() => Ok(val),
        Ok(_) => Err(EnvVarError::new(
            1002,
            &format!("El valor de '{}' está vacío", var_name),
            EnvVarErrorKind::Invalid,
            var_name,
            "env.rs",
        )),
        Err(_) => Err(EnvVarError::new(
            1001,
            &format!("'{}' no está definido", var_name),
            EnvVarErrorKind::Missing,
            var_name,
            "env.rs",
        )),
    }
}

fn get_env_port(var_name: &str) -> Result<u16, EnvVarError> {
    let val = match env::var(var_name) {
        Ok(v) => v,
        Err(_) => {
            return Err(EnvVarError::new(
                1001,
                &format!("'{}' no está definido", var_name),
                EnvVarErrorKind::Missing,
                var_name,
                "env.rs",
            ));
        }
    };

    val.parse::<u16>().map_err(|_| {
        EnvVarError::new(
            1003,
            &format!("'{}' inválido (debe ser 0-65535)", var_name),
            EnvVarErrorKind::Invalid,
            var_name,
            "env.rs",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    // SAFETY: Los tests corren secuencialmente gracias a #[serial],
    // por lo que no hay data races al modificar variables de entorno.
    unsafe fn cleanup_env() {
        unsafe {
            remove_env("HOST");
            remove_env("PORT");
            remove_env("DB_USER");
            remove_env("DB_PASSWORD");
            remove_env("DB_HOST");
            remove_env("DB_PORT");
            remove_env("DB_NAME");
            remove_env("DB_REPLICA_SET");
            remove_env("DB_AUTH_SOURCE");
        }
    }

    unsafe fn set_env(key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
    }

    unsafe fn remove_env(key: &str) {
        unsafe {
            env::remove_var(key);
        }
    }

    unsafe fn set_all_env() {
        unsafe {
            set_env("HOST", "localhost");
            set_env("PORT", "3306");
            set_env("DB_USER", "testuser");
            set_env("DB_PASSWORD", "testpass");
            set_env("DB_HOST", "127.0.0.1");
            set_env("DB_PORT", "27017");
            set_env("DB_NAME", "testdb");
            set_env("DB_REPLICA_SET", "rs0");
            set_env("DB_AUTH_SOURCE", "admin");
        }
    }

    #[test]
    #[serial]
    fn load_config_success() {
        unsafe {
            cleanup_env();
            set_all_env();
        }

        let config = load_config_from_env().unwrap();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 3306);
        assert_eq!(config.db_user, "testuser");
        assert_eq!(config.db_password, "testpass");
        assert_eq!(config.db_host, "127.0.0.1");
        assert_eq!(config.db_port, 27017);
        assert_eq!(config.db_name, "testdb");
        assert_eq!(config.db_replica_set, "rs0");
        assert_eq!(config.db_auth_source, "admin");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_mongo_url_format() {
        unsafe {
            cleanup_env();
            set_all_env();
        }

        let config = load_config_from_env().unwrap();
        let url = config.mongo_url();

        assert!(url.contains("testuser"));
        assert!(url.contains("testpass"));
        assert!(url.contains("127.0.0.1"));
        assert!(url.contains("27017"));
        assert!(url.contains("rs0"));
        assert!(url.contains("admin"));

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_host() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("HOST");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "HOST");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_empty_host() {
        unsafe {
            cleanup_env();
            set_all_env();
            set_env("HOST", "   ");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Invalid);
        assert_eq!(err.var_name(), "HOST");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_port() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("PORT");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "PORT");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_invalid_port() {
        unsafe {
            cleanup_env();
            set_all_env();
            set_env("PORT", "no_es_numero");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Invalid);
        assert_eq!(err.var_name(), "PORT");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_port_out_of_range() {
        unsafe {
            cleanup_env();
            set_all_env();
            set_env("PORT", "99999");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Invalid);

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_user() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_USER");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_USER");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_empty_db_user() {
        unsafe {
            cleanup_env();
            set_all_env();
            set_env("DB_USER", "  ");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Invalid);
        assert_eq!(err.var_name(), "DB_USER");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_password() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_PASSWORD");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_PASSWORD");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_host() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_HOST");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_HOST");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_port() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_PORT");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_PORT");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_invalid_db_port() {
        unsafe {
            cleanup_env();
            set_all_env();
            set_env("DB_PORT", "invalid");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Invalid);
        assert_eq!(err.var_name(), "DB_PORT");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_name() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_NAME");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_NAME");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_replica_set() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_REPLICA_SET");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_REPLICA_SET");

        unsafe { cleanup_env() };
    }

    #[test]
    #[serial]
    fn load_config_missing_db_auth_source() {
        unsafe {
            cleanup_env();
            set_all_env();
            remove_env("DB_AUTH_SOURCE");
        }

        let result = load_config_from_env();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), &EnvVarErrorKind::Missing);
        assert_eq!(err.var_name(), "DB_AUTH_SOURCE");

        unsafe { cleanup_env() };
    }
}
