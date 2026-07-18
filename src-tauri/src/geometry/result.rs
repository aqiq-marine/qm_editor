use serde::{Deserialize, Serialize};
use specta::Type;

use super::artifact::GeometryArtifactRef;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryResult {
    pub primary_structure: Option<GeometryStructure>,
    pub conformers: Vec<ConformerResult>,
    pub energies: Vec<EnergyRecord>,
    pub run: Option<GeometryRunSummary>,
    pub metadata: GeometryResultMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryStructure {
    pub artifact: GeometryArtifactRef,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ConformerResult {
    pub artifact: GeometryArtifactRef,
    pub energy: Option<f64>,
    pub rank: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EnergyRecord {
    pub label: Option<String>,
    pub value: f64,
    pub unit: EnergyUnit,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnergyUnit {
    Hartree,
    KcalMol,
    KjMol,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryResultMetadata {
    pub engine_id: super::GeometryEngineId,
    pub notes: Vec<String>,
    pub warnings: Vec<GeometryRuntimeWarning>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryRunSummary {
    pub iteration_count: Option<u64>,
    pub elapsed_ms: Option<u64>,
    pub termination: GeometryTermination,
    pub warnings: Vec<GeometryRuntimeWarning>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryTermination {
    Converged,
    MaxIterationsReached,
    NumericalFailure,
    Cancelled,
    CompletedWithoutConvergence,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryRuntimeWarning {
    pub code: String,
    pub message: String,
}
