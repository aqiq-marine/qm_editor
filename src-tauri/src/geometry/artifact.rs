use serde::{Deserialize, Serialize};
use specta::Type;

use super::ArtifactId;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryArtifactRef {
    pub id: ArtifactId,
    pub kind: GeometryArtifactKind,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryArtifactKind {
    Structure,
    ConformerSet,
    ScanProfile,
    EnergyProfile,
    Intermediate,
    Custom { name: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryArtifact {
    pub reference: GeometryArtifactRef,
    pub label: Option<String>,
    pub metadata_json: Option<String>,
}
