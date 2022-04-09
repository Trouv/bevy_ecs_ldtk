// This example uses the LevelSet component instead of the LevelSelection resource.
// The setup system puts every level uid in the LevelSet, so the entire LDtk world is spawned.

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(toggle_levels)
        // No LevelSelection resource!
        .insert_resource(LdtkSettings {
            // By default, levels are just spawned at the origin of the world.
            // This makes them spawn according to their location in LDtk
            use_level_world_translations: true,
            ..Default::default()
        })
        .run();
}

const LEVEL_UIDS: [i32; 9] = [3, 40, 0, 2, 4, 29, 30, 41, 42];

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("WorldMap_Free_layout.ldtk"),
        level_set: LevelSet {
            iids: HashSet::from(LEVEL_UIDS),
        },
        transform: Transform::from_xyz(-232., -496., 0.),
        ..Default::default()
    });
}

// This function is a demonstation that changes to the LevelSet have the expected results.
// Hit spacebar and watch what happens!
fn toggle_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
    if input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let level_to_toggle = LEVEL_UIDS.choose(&mut rng).unwrap();

        let mut level_set = level_sets.single_mut();
        if level_set.iids.contains(level_to_toggle) {
            level_set.iids.remove(level_to_toggle);
        } else {
            level_set.iids.insert(*level_to_toggle);
        }
    }
}
