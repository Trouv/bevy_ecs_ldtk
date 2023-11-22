use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .add_systems(
            Update,
            (
                move_player_from_input,
                translate_grid_coords,
                cache_wall_locations,
                check_goal,
            ),
        )
        .init_resource::<WallLocations>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
struct WallLocations(HashSet<GridCoords>);

fn move_player_from_input() {}

fn translate_grid_coords() {}

fn cache_wall_locations() {}

fn check_goal() {}
