use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryExecutionProgress {
    pub stage: GeometryExecutionStage,
    pub phase: Option<String>,
    pub fraction: Option<f32>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryExecutionStage {
    Preparing,
    Embedding,
    Optimizing,
    Scanning,
    Evaluating,
    WritingOutput,
    Completed,
}
