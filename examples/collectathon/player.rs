use crate::coin::Wallet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    wallet: Wallet,
    #[sprite_sheet_bundle]
    sprite_sheet: SpriteSheetBundle,
}
