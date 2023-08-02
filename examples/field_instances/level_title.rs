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
    level_handles: Query<&Handle<LdtkExternalLevel>>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
    mut current_level_title: ResMut<LevelTitle>,
) {
    for level_event in level_events.iter() {
        if matches!(level_event, LevelEvent::Transformed(_)) {
            let level_handle = level_handles
                .get_single()
                .expect("only one level should be spawned at a time in this example");

            let level_asset = level_assets
                .get(level_handle)
                .expect("level asset should be loaded before LevelEvent::Transformed");

            let title = level_asset
                .data()
                .get_string_field("title")
                .expect("level should have non-nullable title string field");

            **current_level_title = title.clone();
        }
    }
}
