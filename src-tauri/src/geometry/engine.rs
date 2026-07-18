use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Arc;

use super::{
    execution::GeometryExecutionContext, validation::GeometryValidationContext, GeometryError,
    GeometryOperation, GeometryResult,
};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEngineDescriptor {
    pub id: super::GeometryEngineId,
    pub display_name: String,
    pub version: String,
}

pub trait GeometryEngine: Send + Sync {
    fn descriptor(&self) -> &GeometryEngineDescriptor;

    fn validate(
        &self,
        operation: &GeometryOperation,
        ctx: &GeometryValidationContext,
    ) -> super::validation::GeometryValidationReport;

    fn execute(
        &self,
        operation: GeometryOperation,
        ctx: &GeometryExecutionContext,
    ) -> Result<GeometryResult, GeometryError>;
}

pub type SharedGeometryEngine = Arc<dyn GeometryEngine>;
