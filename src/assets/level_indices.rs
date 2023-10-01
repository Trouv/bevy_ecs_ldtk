use bevy::reflect::Reflect;

/// Indices pointing to the location of a level in an [`LdtkProject`] or [`LdtkJson`].
///
/// This type supports multi-world projects by storing an optional `world` index.
/// If this is present, the level index is used within that world.
/// If not, the level index is used in the project root's level collection.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
/// [`LdtkJson`]: crate::ldtk::LdtkJson
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct LevelIndices {
    /// The index of the world the level belongs to, if the project is multi-world.
    pub world: Option<usize>,
    /// The index of the level, either within a world or in the root of the project.
    pub level: usize,
}

impl LevelIndices {
    /// Construct a new [`LevelIndices`] pointing to a level in a world.
    ///
    /// # Example
    /// ```
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// let level_indices = LevelIndices::in_world(1, 2);
    ///
    /// assert_eq!(level_indices, LevelIndices { world: Some(1), level: 2 });
    /// ```
    pub fn in_world(world_index: usize, level_index: usize) -> LevelIndices {
        LevelIndices {
            world: Some(world_index),
            level: level_index,
        }
    }

    /// Construct a new [`LevelIndices`] pointing to a level in the project root.
    ///
    /// # Example
    /// ```
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// let level_indices = LevelIndices::in_root(3);
    ///
    /// assert_eq!(level_indices, LevelIndices { world: None, level: 3 });
    /// ```
    pub fn in_root(index: usize) -> LevelIndices {
        LevelIndices {
            world: None,
            level: index,
        }
    }
}
