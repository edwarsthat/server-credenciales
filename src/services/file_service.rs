use std::path::{Component, Path, PathBuf};
use tokio::fs;

const STORAGE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../storage");

pub enum StorageFolder {
    Root,
    Foto,
    Identificacion,
}

impl StorageFolder {
    fn subfolder(&self) -> &'static str {
        match self {
            StorageFolder::Root => "",
            StorageFolder::Foto => "foto",
            StorageFolder::Identificacion => "identificacion",
        }
    }
}

pub enum FileValidationError {
    EmptyPath,
    PathTraversal,
    OutsideBaseDir,
    NotAFile,
    NotFound,
    AccessError,
}

impl FileValidationError {
    pub fn message(&self) -> &str {
        match self {
            FileValidationError::EmptyPath => "Ruta de archivo no proporcionada",
            FileValidationError::PathTraversal => "Acceso denegado: fuera del directorio permitido",
            FileValidationError::OutsideBaseDir => "Ruta de archivo fuera del directorio permitido",
            FileValidationError::NotAFile => "La ruta no pertenece a un archivo válido",
            FileValidationError::NotFound => "Archivo no encontrado",
            FileValidationError::AccessError => "Error de acceso al archivo",
        }
    }
}

pub struct FileService;

impl FileService {
    pub fn new() -> Self {
        FileService
    }

    pub async fn validate_file_path(
        &self,
        filename: &str,
        folder: StorageFolder,
    ) -> Result<PathBuf, FileValidationError> {
        let filename = filename.trim();

        if filename.is_empty() {
            return Err(FileValidationError::EmptyPath);
        }

        let base_path = PathBuf::from(STORAGE_PATH).join(folder.subfolder());

        // Rechazar caracteres prohibidos antes de cualquier parseo
        if filename.contains('\0') || filename.contains('\\') {
            return Err(FileValidationError::PathTraversal);
        }

        // Verificar que ningún componente sea ".." o ruta absoluta (path traversal)
        let input_path = Path::new(filename);
        for component in input_path.components() {
            match component {
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(FileValidationError::PathTraversal);
                }
                _ => {}
            }
        }

        // Resolver la ruta final
        let resolved = base_path.join(input_path);

        // Canonicalizar resuelve symlinks y ".." reales — falla si el archivo no existe
        let canonical_base = fs::canonicalize(&base_path).await
            .map_err(|_| FileValidationError::AccessError)?;

        let canonical_resolved = fs::canonicalize(&resolved).await
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => FileValidationError::NotFound,
                _ => FileValidationError::AccessError,
            })?;

        // Verificar que la ruta real esté dentro del directorio base (resuelve symlinks)
        if !canonical_resolved.starts_with(&canonical_base) {
            return Err(FileValidationError::OutsideBaseDir);
        }

        // Verificar que sea un archivo y no un directorio u otro tipo
        if !canonical_resolved.is_file() {
            return Err(FileValidationError::NotAFile);
        }

        Ok(canonical_resolved)
    }

    fn content_type(&self, filename: &str) -> Option<&'static str> {
        match Path::new(filename).extension().and_then(|e| e.to_str()) {
            Some("jpg") | Some("jpeg") => Some("image/jpeg"),
            Some("png") => Some("image/png"),
            Some("webp") => Some("image/webp"),
            _ => None,
        }
    }

    pub async fn read_file(
        &self,
        filename: &str,
        folder: StorageFolder,
    ) -> Result<(Vec<u8>, &'static str), FileValidationError> {
        let content_type = self.content_type(filename)
            .ok_or(FileValidationError::PathTraversal)?;

        let path = self.validate_file_path(filename, folder).await?;
        let bytes = fs::read(&path).await.map_err(|_| FileValidationError::AccessError)?;

        Ok((bytes, content_type))
    }
}
