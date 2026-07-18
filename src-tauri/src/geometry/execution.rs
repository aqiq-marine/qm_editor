use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    progress::GeometryExecutionProgress, validation::GeometryValidationReport,
    GeometryArtifactRef, GeometryError, GeometryEngineDescriptor, GeometryOperation, GeometryResult,
};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryExecutionContext {
    pub metadata: std::collections::HashMap<String, String>,
}

impl GeometryExecutionContext {
    pub fn new() -> Self {
        Self {
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryOperationExecution {
    pub operation: GeometryOperation,
    pub resolved_engine: Option<GeometryEngineDescriptor>,
    pub state: GeometryExecutionState,
    pub validation: Option<GeometryValidationReport>,
    pub progress: Option<GeometryExecutionProgress>,
    pub started_at_ms: Option<u64>,
    pub finished_at_ms: Option<u64>,
    pub result: Option<GeometryResult>,
    pub failure: Option<GeometryError>,
    pub artifacts: Vec<GeometryArtifactRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryExecutionState {
    Draft,
    Validated,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryExecutionEvent {
    Progress(GeometryExecutionProgress),
    Warning(super::result::GeometryRuntimeWarning),
    Log { level: LogLevel, message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
