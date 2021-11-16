use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod assets;
pub mod bundler;
pub mod components;
pub mod ldtk;
mod systems;
pub use bundler::AddBundle;

#[derive(Clone, Eq, PartialEq, Debug, Component)]
pub enum LevelSelection {
    Identifier(String),
    Index(usize),
    Uid(i64),
}

impl Default for LevelSelection {
    fn default() -> Self {
        LevelSelection::Index(0)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkPlugin;

#[derive(Clone, Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_handle: Handle<assets::LdtkAsset>,
    pub level_selection: LevelSelection,
    pub map: Map,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<assets::LdtkAsset>()
            .init_asset_loader::<assets::LdtkLoader>()
            .add_asset::<assets::LdtkExternalLevel>()
            .init_asset_loader::<assets::LdtkLevelLoader>()
            .add_system(systems::process_external_levels)
            .add_system(systems::process_loaded_ldtk);
    }
}
