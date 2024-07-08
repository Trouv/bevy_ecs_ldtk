//! Contains [LevelTitle] and the system that sets its value.
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Resource storing the current level's title.
///
/// This is sourced from the "title" field of the level in LDtk.
#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct LevelTitle(String);

pub fn set_level_title_to_current_level(
    mut level_events: EventReader<LevelEvent>,
    levels: Query<&LevelIid>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    mut current_level_title: ResMut<LevelTitle>,
) {
    for level_event in level_events.read() {
        if matches!(level_event, LevelEvent::Transformed(_)) {
            let level_iid = levels
                .get_single()
                .expect("only one level should be spawned at a time in this example");

            let level_data = project_assets
                .get(projects.single())
                .expect("project asset should be loaded if levels are spawned")
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("spawned level should exist in the loaded project");

            let title = level_data
                .get_string_field("title")
                .expect("level should have non-nullable title string field");

            (*current_level_title).clone_from(title);
        }
    }
}
