//! Resources and events used by the plugin.

use crate::ldtk::Level;

#[allow(unused_imports)]
use bevy::prelude::{CoreStage, GlobalTransform};

#[allow(unused_imports)]
use crate::components::{LdtkWorldBundle, LevelSet};

/// Resource for choosing which level(s) to spawn.
///
/// Updating this will despawn the current level and spawn the new one (unless they are the same).
/// You can also load the selected level's neighbors using the [LevelSpawnBehavior] option.
///
/// This resource works by updating the [LdtkWorldBundle]'s [LevelSet] component.
/// If you need more control over the spawned levels than this resource provides,
/// you can choose not to insert this resource and interface with [LevelSet] directly instead.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LevelSelection {
    /// Spawn level with the given identifier.
    Identifier(String),
    /// Spawn level from its index in the LDtk file's list of levels.
    Index(usize),
    /// Spawn level with the given level `iid`.
    Iid(String),
    /// Spawn level with the given level `uid`.
    Uid(i32),
}

impl Default for LevelSelection {
    fn default() -> Self {
        LevelSelection::Index(0)
    }
}

impl LevelSelection {
    pub fn is_match(&self, index: &usize, level: &Level) -> bool {
        match self {
            LevelSelection::Identifier(s) => *s == level.identifier,
            LevelSelection::Index(i) => *i == *index,
            LevelSelection::Iid(i) => *i == level.iid,
            LevelSelection::Uid(u) => *u == level.uid,
        }
    }
}

/// Option in [LdtkSettings] that determines clear color behavior.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SetClearColor {
    /// Don't update the clear color at all
    No,
    /// Update the clear color to use the background color of the current level
    /// (determined by [LevelSelection])
    FromLevelBackground,
    /// Update the clear color to use the entire editor's background color
    FromEditorBackground,
}

impl Default for SetClearColor {
    fn default() -> Self {
        Self::No
    }
}

/// Option in [LdtkSettings] that determines level spawn behavior.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LevelSpawnBehavior {
    /// Newly spawned levels will be spawned with a translation of zero relative to the
    /// [LdtkWorldBundle].
    UseZeroTranslation,
    /// Newly spawned levels will be spawned with translations like their location in the LDtk
    /// world.
    ///
    /// Useful for "2d free map" and "GridVania" layouts.
    UseWorldTranslation {
        /// When used with the [LevelSelection] resource, levels in the `__level_neighbors` list of
        /// the selected level will be spawned in addition to the selected level.
        load_level_neighbors: bool,
    },
}

impl Default for LevelSpawnBehavior {
    fn default() -> Self {
        LevelSpawnBehavior::UseZeroTranslation
    }
}

/// Option in [LdtkSettings] that determines the visual representation of IntGrid layers when they don't have AutoTile rules.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IntGridRendering {
    /// Renders the tile with its corresponding color in LDtk, so it appears like it does in LDtk
    Colorful,
    /// Does not render the tile
    Invisible,
}

impl Default for IntGridRendering {
    fn default() -> Self {
        IntGridRendering::Colorful
    }
}

/// Option in [LdtkSettings] that dictates how the plugin handles level backgrounds.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LevelBackground {
    /// The level background's color (and image, if it exists) are rendered.
    /// The first layer of the level will be the background color.
    Rendered,
    /// There will be no level backgrounds, not even an empty layer.
    Nonexistent,
}

impl Default for LevelBackground {
    fn default() -> Self {
        LevelBackground::Rendered
    }
}

/// Settings resource for the plugin.
/// Check out the documentation for each field type to learn more.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkSettings {
    pub level_spawn_behavior: LevelSpawnBehavior,
    pub set_clear_color: SetClearColor,
    pub int_grid_rendering: IntGridRendering,
    pub level_background: LevelBackground,
}

/// Events fired by the plugin related to level spawning/despawning.
///
/// Each variant stores the level's `iid` in LDtk.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelEvent {
    /// Indicates that a level has been triggered to spawn, but hasn't been spawned yet.
    SpawnTriggered(String),
    /// The level, with all of its layers, entities, etc., has spawned.
    ///
    /// Note: due to the frame-delay of [GlobalTransform] being updated, this may not be the event
    /// you want to listen for.
    /// If your systems are [GlobalTransform]-dependent, see [LevelEvent::Transformed].
    Spawned(String),
    /// Occurs during the [CoreStage::PostUpdate] after the level has spawned, so all
    /// [GlobalTransform]s of the level should be updated.
    Transformed(String),
    /// Indicates that a level has despawned.
    Despawned(String),
}
