use serde::{Deserialize, Serialize};
use specta::Type;

use super::CoordinateFrameRef;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryOptions {
    pub max_iterations: Option<u32>,
    pub convergence: Option<ConvergencePolicy>,
    pub allow_approximation: bool,
    pub coordinate_frame: Option<CoordinateFrameRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ConvergencePolicy {
    pub energy_tolerance: Option<f64>,
    pub gradient_tolerance: Option<f64>,
    pub rms_gradient_tolerance: Option<f64>,
}
