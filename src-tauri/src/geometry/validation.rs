use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryValidationContext {
    pub metadata: std::collections::HashMap<String, String>,
}

impl GeometryValidationContext {
    pub fn new() -> Self {
        Self {
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryValidationReport {
    pub status: GeometryValidationStatus,
    pub issues: Vec<GeometryValidationIssue>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryValidationStatus {
    Valid,
    Invalid,
    Unsupported,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryValidationIssue {
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    pub applicability: ValidationApplicability,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationApplicability {
    Exact,
    Approximate,
    Unsupported,
}
