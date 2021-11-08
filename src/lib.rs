use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct LevelIdentifier {
    pub identifier: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin);
    }
}
