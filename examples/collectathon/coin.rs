use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Plugin for spawning coins and collecting them.
pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collect)
            .register_ldtk_entity::<CoinBundle>("Coin");
    }
}

/// Component marking coin entities.
#[derive(Default, Component)]
struct Coin;

#[derive(Default, Bundle, LdtkEntity)]
struct CoinBundle {
    coin: Coin,
    #[sprite_sheet_bundle]
    sprite_sheet: LdtkSpriteSheetBundle,
}

/// Component for entities that can collect coins.
/// Stores the number of coins they have collected.
#[derive(Default, Component)]
pub struct Wallet {
    coins: u32,
}

const COLLECT_DISTANCE: f32 = 12.;

fn collect(
    mut commands: Commands,
    mut wallets: Query<(&mut Wallet, &GlobalTransform)>,
    coins: Query<(Entity, &GlobalTransform), With<Coin>>,
) {
    for (mut wallet, wallet_transform) in wallets.iter_mut() {
        for (coin_entity, coin_transform) in coins.iter() {
            // Global translations of new entities will always be 0 for one update.
            // This check prevents wallets near 0 collecting newly-spawned coins.
            if coin_transform.translation() == Vec3::ZERO {
                continue;
            }

            let distance = wallet_transform
                .translation()
                .distance(coin_transform.translation());

            if distance <= COLLECT_DISTANCE {
                wallet.coins += 1;
                println!("Coins: {}", wallet.coins);

                commands.entity(coin_entity).despawn_recursive();
            }
        }
    }
}
