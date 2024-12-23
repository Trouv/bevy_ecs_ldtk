use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    commands.spawn(Camera2d);

    rapier_config.single_mut().gravity = Vec2::new(0.0, -2000.0);

    let ldtk_handle = asset_server
        .load("Typical_2D_platformer_example.ldtk")
        .into();
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

pub fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (level_iid, level_transform) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + level.px_wid as f32,
                level_transform.translation.y + level.px_hei as f32,
            ),
        };

        for player_transform in &player_query {
            if player_transform.translation.x < level_bounds.max.x
                && player_transform.translation.x > level_bounds.min.x
                && player_transform.translation.y < level_bounds.max.y
                && player_transform.translation.y > level_bounds.min.y
                && !level_selection.is_match(&LevelIndices::default(), level)
            {
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

pub fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for level_entity in &level_query {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_level_selection)
            .add_systems(Update, restart_level);
    }
}
