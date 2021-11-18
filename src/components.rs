pub use crate::ldtk::EntityInstance;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i64,
}

#[derive(Clone, Default, Bundle)]
pub struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
    #[bundle]
    pub tile_bundle: TileBundle,
}

impl TileBundleTrait for IntGridCellBundle {
    fn get_tile_pos_mut(&mut self) -> &mut TilePos {
        &mut self.tile_bundle.position
    }

    fn get_tile_parent(&mut self) -> &mut TileParent {
        &mut self.tile_bundle.parent
    }
}

#[derive(Clone, Bundle, Default)]
pub struct EntityInstanceBundle {
    pub entity_instance: EntityInstance,
}

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

#[derive(Clone, Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_handle: Handle<crate::assets::LdtkAsset>,
    pub level_selection: LevelSelection,
    pub map: Map,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
