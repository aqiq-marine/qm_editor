use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    input::GeometryInput, constraint::GeometryConstraint, objective::GeometryObjective,
    options::GeometryOptions, GeometryEngineId, OperationId,
};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryOperation {
    pub id: OperationId,
    pub input: GeometryInput,
    pub engine: GeometryEngineId,
    pub objective: GeometryObjective,
    pub constraints: Vec<GeometryConstraint>,
    pub options: GeometryOptions,
}
