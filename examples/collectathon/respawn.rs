use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Plugin for respawning levels and worlds.
pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (respawn_level, respawn_world));
    }
}

fn respawn_level(
    mut commands: Commands,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyL) {
        let level_selection_iid = match level_selection.as_ref() {
            LevelSelection::Iid(iid) => iid,
            _ => panic!("level should always be selected by iid in this example"),
        };

        for (level_entity, level_iid) in levels.iter() {
            if level_iid == level_selection_iid {
                commands.entity(level_entity).insert(Respawn);
            }
        }
    }
}

fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<Handle<LdtkProject>>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands.entity(ldtk_projects.single()).insert(Respawn);
    }
}
