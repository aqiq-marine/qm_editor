use serde::{Deserialize, Serialize};
use specta::Type;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
        #[serde(transparent)]
        pub struct $name(pub String);
    };
}

id_type!(OperationId);
id_type!(ArtifactId);
id_type!(GeometryEngineId);
id_type!(MoleculeRef);
id_type!(ConformerRef);
id_type!(CoordinateFrameRef);
id_type!(AtomRef);
