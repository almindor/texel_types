use crate::{Position, Position2D, Sprite, SpriteV1};
use std::collections::BTreeMap;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

///
/// Previous scene version == V1
///
/// ### Contents
/// SceneV1 consists of a list of tuples each having:
/// * SpriteV1
/// * Position
/// * bool (selected indicator) -- *DEPRECATED!*
///
/// #### WARNING
/// The selected indicator boolean will get removed in V2
/// as the only "lossy" change of the Scene format.
/// Leaving this in SceneV1 definition was a mistake during initial refactoring.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SceneV1 {
    pub objects: Vec<(SpriteV1, Position, bool)>,
}

///
/// Current scene version == V2
///
/// ### Contents
/// SceneV2 consists of a list of tuples each having:
/// * Sprite
/// * Position (for sprite)
/// SceneV2 also consists of a list of:
/// * Position2D (for bookmarks)
///
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct SceneV2 {
    pub objects: Vec<(Sprite, Position)>,
    pub bookmarks: BTreeMap<usize, Position2D>,
}

impl From<SceneV1> for SceneV2 {
    fn from(older: SceneV1) -> Self {
        let mut objects: Vec<(Sprite, Position)> = Vec::with_capacity(older.objects.capacity());

        for obj in older.objects {
            objects.push((Sprite::from(obj.0), obj.1))
        }

        SceneV2 { objects, bookmarks: BTreeMap::new() }
    }
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
    V2(SceneV2),
}

impl Default for Scene {
    fn default() -> Self {
        Scene::V2(SceneV2::default())
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
    /// * `SceneV2` - current scene version
    ///
    pub fn current(self) -> SceneV2 {
        match self {
            Self::V2(scene) => scene,
            Self::V1(scene) => SceneV2::from(scene),
        }
    }
}
