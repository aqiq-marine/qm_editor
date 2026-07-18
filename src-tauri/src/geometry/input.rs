use serde::{Deserialize, Serialize};
use specta::Type;

use super::{artifact::GeometryArtifactRef, ConformerRef, CoordinateFrameRef, MoleculeRef};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryInput {
    pub source: GeometryInputSource,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryInputSource {
    Molecule(MoleculeRef),
    Conformer(ConformerRef),
    Artifact(GeometryArtifactRef),
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryInputContext {
    pub coordinate_frame: Option<CoordinateFrameRef>,
}
