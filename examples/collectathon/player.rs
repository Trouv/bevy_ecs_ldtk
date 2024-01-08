use crate::coin::Wallet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Plugin for spawning the player and controlling them.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, level_selection_follow_player))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

/// Component marking the player entity.
#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    wallet: Wallet,
    #[worldly]
    worldly: Worldly,
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
            movement -= Vec2::X;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            movement -= Vec2::Y;
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

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for player_transform in players.iter() {
        if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) {
            for (level_iid, level_transform) in levels.iter() {
                let level = ldtk_project
                    .get_raw_level_by_iid(level_iid.get())
                    .expect("level should exist in only project");

                let level_bounds = Rect {
                    min: Vec2::new(
                        level_transform.translation().x,
                        level_transform.translation().y,
                    ),
                    max: Vec2::new(
                        level_transform.translation().x + level.px_wid as f32,
                        level_transform.translation().y + level.px_hei as f32,
                    ),
                };

                if level_bounds.contains(player_transform.translation().truncate()) {
                    *level_selection = LevelSelection::Iid(level_iid.clone());
                }
            }
        }
    }
}
