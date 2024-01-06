use crate::coin::Wallet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player)
            .register_ldtk_entity::<PlayerBundle>("Player");
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

const MOVEMENT_SPEED: f32 = 96.;

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut player_transform in players.iter_mut() {
        let mut movement = Vec2::ZERO;

        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            movement += Vec2::Y;
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            movement += Vec2::NEG_X;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            movement += Vec2::NEG_Y;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            movement += Vec2::X;
        }

        if movement != Vec2::ZERO {
            player_transform.translation +=
                movement.extend(0.) * MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}
