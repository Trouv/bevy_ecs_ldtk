use crate::{ldtk::Level, LevelIid};
use bevy::prelude::*;

/// [`Resource`] for choosing which level(s) to spawn.
///
/// Updating this will despawn the current level and spawn the new one (unless they are the same).
/// You can also load the selected level's neighbors using the [`LevelSpawnBehavior`] option.
///
/// This resource works by updating the [`LdtkWorldBundle`]'s [`LevelSet`] component.
/// If you need more control over the spawned levels than this resource provides,
/// you can choose not to insert this resource and interface with [`LevelSet`] directly instead.
///
/// [`LevelSpawnBehavior`]: crate::prelude::LevelSpawnBehavior
/// [`LdtkWorldBundle`]: crate::prelude::LdtkWorldBundle
/// [`LevelSet`]: crate::prelude::LevelSet
/// [`Resource`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.Resource.html
#[derive(Clone, Eq, PartialEq, Debug, Resource)]
pub enum LevelSelection {
    /// Spawn level with the given identifier.
    Identifier(String),
    /// Spawn level from its index in the LDtk file's list of levels.
    Index(usize),
    /// Spawn level with the given level `iid`.
    Iid(LevelIid),
    /// Spawn level with the given level `uid`.
    Uid(i32),
}

impl Default for LevelSelection {
    fn default() -> Self {
        LevelSelection::Index(0)
    }
}

impl LevelSelection {
    /// Construct a [`LevelSelection::Iid`] using the given iid.
    ///
    /// This iid only needs to implement `Into<String>`.
    ///
    /// # Example
    /// ```
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// let level_selection = LevelSelection::iid("333081f6-7ac1-4fe3-bdcc-fa3704073bbe");
    /// # assert_eq!(
    /// #     level_selection,
    /// #     LevelSelection::Iid(LevelIid::new("333081f6-7ac1-4fe3-bdcc-fa3704073bbe"))
    /// # );
    /// ```
    pub fn iid(iid: impl Into<String>) -> Self {
        LevelSelection::Iid(LevelIid::new(iid))
    }

    /// Return true if the given level matches this [`LevelSelection`].
    ///
    /// Since levels don't inherently store their index, it needs to be provided separately.
    pub fn is_match(&self, index: &usize, level: &Level) -> bool {
        match self {
            LevelSelection::Identifier(s) => *s == level.identifier,
            LevelSelection::Index(i) => *i == *index,
            LevelSelection::Iid(i) => *i.get() == level.iid,
            LevelSelection::Uid(u) => *u == level.uid,
        }
    }
}
