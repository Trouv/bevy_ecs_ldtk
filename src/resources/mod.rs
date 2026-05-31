//! Resources and events used by the plugin.
use bevy::prelude::*;

#[allow(unused_imports)]
use crate::assets::LdtkProject;
#[allow(unused_imports)]
use crate::components::LdtkWorldBundle;

mod level_selection;
pub use level_selection::LevelSelection;

mod level_event;
pub use level_event::LevelEvent;

/// Option in [LdtkSettings] that determines clear color behavior.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum SetClearColor {
    /// Don't update the clear color at all
    #[default]
    No,
    /// Update the clear color to use the background color of the current level
    /// (determined by [`LevelSelection`])
    FromLevelBackground,
    /// Update the clear color to use the entire editor's background color
    FromEditorBackground,
}

/// Option in [LdtkSettings] that determines level spawn behavior.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum LevelSpawnBehavior {
    /// Newly spawned levels will be spawned with a translation of zero relative to the
    /// [LdtkWorldBundle].
    #[default]
    UseZeroTranslation,
    /// Newly spawned levels will be spawned with translations like their location in the LDtk
    /// world.
    ///
    /// Useful for "2d free map" and "GridVania" layouts.
    UseWorldTranslation {
        /// When used with the [`LevelSelection`] resource, levels in the `__level_neighbors` list
        /// of the selected level will be spawned in addition to the selected level.
        load_level_neighbors: bool,
    },
}

/// Option in [LdtkSettings] that determines the visual representation of IntGrid layers when they don't have AutoTile rules.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum IntGridRendering {
    /// Renders the tile with its corresponding color in LDtk, so it appears like it does in LDtk
    #[default]
    Colorful,
    /// Does not render the tile
    Invisible,
}

/// Option in [LdtkSettings] that dictates how the plugin handles level backgrounds.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum LevelBackground {
    /// The level background's color (and image, if it exists) are rendered.
    /// The first layer of the level will be the background color.
    #[default]
    Rendered,
    /// There will be no level backgrounds, not even an empty layer.
    Nonexistent,
}

/// Specifies data that should be ignored completely when spawning levels. Excluded items will still
/// be present in the [`LdtkProject`] but will not cause any entities to be spawned in the world.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct SpawnExclusions {
    /// List of layer `Identifier` names (not UIDs) to skip spawning as tilemaps.
    pub layer_identifiers: Vec<String>,
}

/// Settings resource for the plugin.
/// Check out the documentation for each field type to learn more.
#[derive(Clone, Eq, PartialEq, Debug, Default, Resource)]
pub struct LdtkSettings {
    pub level_spawn_behavior: LevelSpawnBehavior,
    pub set_clear_color: SetClearColor,
    pub int_grid_rendering: IntGridRendering,
    pub level_background: LevelBackground,
    pub exclusions: SpawnExclusions,
}
