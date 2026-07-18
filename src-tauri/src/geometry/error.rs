use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Clone, Debug, Error, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum GeometryError {
    #[error("invalid operation: {message}")]
    InvalidOperation { message: String },
    #[error("unsupported operation: {message}")]
    UnsupportedOperation { message: String },
    #[error("engine not found: {message}")]
    EngineNotFound { message: String },
    #[error("execution failed: {message}")]
    ExecutionFailed { message: String },
    #[error("validation failed: {message}")]
    ValidationFailed { message: String },
    #[error("registry error: {message}")]
    RegistryError { message: String },
}
