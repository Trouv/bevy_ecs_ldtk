use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CoinBundle>("Coin");
    }
}

#[derive(Default, Component)]
struct Coin;

#[derive(Default, Bundle, LdtkEntity)]
struct CoinBundle {
    coin: Coin,
    #[sprite_sheet_bundle]
    sprite_sheet: SpriteSheetBundle,
}

#[derive(Default, Component)]
pub struct Wallet {
    coins: u32,
}
