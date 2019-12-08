use crate::{Position, Sprite};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

///
/// Current scene version == V1
/// 
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SceneV1 {
    pub objects: Vec<(Sprite, Position, bool)>,
}

///
/// Scene is the final serialization artifact for texel_types.
/// As such it needs to be versioned explicitly so it can be known which version
/// of the serialized scene we're deserializing from files. This enum wrapper
/// will hold any version of the scene object to provide forward compatibility.
/// 
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Scene {
    V1(SceneV1),
}

impl Default for Scene {
    fn default() -> Self {
        Scene::V1(SceneV1::default())
    }
}

impl Scene {
    ///
    /// Method used to retreive the current version of scene from the enum wrapper.
    /// This method will conver previous versions of scenes to the current one and
    /// return the contents.
    ///
    /// # Returns
    ///
    /// * `SceneV1` - current scene version
    ///
    pub fn current(self) -> SceneV1 {
        match self {
            Self::V1(scene) => scene,
            // TODO: once we have V2+ we'll need to return that and convert previous
        }
    }
}