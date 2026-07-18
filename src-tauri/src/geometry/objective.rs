use serde::{Deserialize, Serialize};
use specta::Type;

use super::AtomRef;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryObjective {
    Build3D,
    Optimize,
    Scan(ScanObjective),
    ConformerSearch,
    Custom { name: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ScanObjective {
    pub coordinate: ScanCoordinate,
    pub range: ScanRange,
    pub step_count: u32,
    pub step_size: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScanCoordinate {
    Bond { atom_ids: [AtomRef; 2] },
    Angle { atom_ids: [AtomRef; 3] },
    Dihedral { atom_ids: [AtomRef; 4] },
    InternalCoordinate { label: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ScanRange {
    pub start: f64,
    pub end: f64,
}
