use crate::app::error::ApiError;
use crate::services::file_service::{FileService, FileValidationError, StorageFolder};

impl From<FileValidationError> for ApiError {
    fn from(e: FileValidationError) -> Self {
        match e {
            FileValidationError::NotFound => ApiError::NotFound(e.message().to_string()),
            FileValidationError::PathTraversal | FileValidationError::OutsideBaseDir => {
                ApiError::BadRequest(e.message().to_string())
            }
            _ => ApiError::InternalError(e.message().to_string()),
        }
    }
}

pub async fn get_foto(filename: &str) -> Result<(Vec<u8>, &'static str), ApiError> {
    let service = FileService::new();
    let (bytes, content_type) = service.read_file(filename, StorageFolder::Root).await?;

    Ok((bytes, content_type))
}
