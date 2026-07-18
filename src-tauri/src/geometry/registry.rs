use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;

use super::{engine::SharedGeometryEngine, GeometryEngineDescriptor, GeometryEngineId};

#[derive(Default)]
pub struct GeometryEngineRegistry {
    engines: HashMap<GeometryEngineId, SharedGeometryEngine>,
}

impl GeometryEngineRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, engine: SharedGeometryEngine) -> Option<SharedGeometryEngine> {
        let id = engine.descriptor().id.clone();
        self.engines.insert(id, engine)
    }

    pub fn get(&self, id: &GeometryEngineId) -> Option<SharedGeometryEngine> {
        self.engines.get(id).cloned()
    }

    pub fn list_descriptors(&self) -> Vec<GeometryEngineDescriptor> {
        self.engines
            .values()
            .map(|engine| engine.descriptor().clone())
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEngineLookup {
    pub engine_id: GeometryEngineId,
}
