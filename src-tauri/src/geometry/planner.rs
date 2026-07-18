use serde::{Deserialize, Serialize};
use specta::Type;

use super::{GeometryOperation, GeometryOperationExecution};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryPlan {
    pub operation: GeometryOperation,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryPlanningResult {
    pub execution: GeometryOperationExecution,
}
