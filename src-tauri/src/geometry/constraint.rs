use serde::{Deserialize, Serialize};
use specta::Type;

use super::AtomRef;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryConstraint {
    AtomFixed { atom: AtomRef },
    CoordinateFixed(GeometryCoordinateConstraint),
    Distance(DistanceConstraint),
    Plane(PlaneConstraint),
    Symmetry(SymmetryConstraint),
    Custom { name: String, payload_json: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeometryCoordinateConstraint {
    Bond {
        atom_ids: [AtomRef; 2],
        value: Option<f64>,
    },
    Angle {
        atom_ids: [AtomRef; 3],
        value: Option<f64>,
    },
    Dihedral {
        atom_ids: [AtomRef; 4],
        value: Option<f64>,
    },
    Position {
        atom: AtomRef,
        position: [f64; 3],
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DistanceConstraint {
    pub atom_ids: [AtomRef; 2],
    pub value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaneConstraint {
    pub atom_ids: Vec<AtomRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SymmetryConstraint {
    pub label: String,
}
