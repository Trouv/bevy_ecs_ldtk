use crate::*;
use bevy::prelude::*;
use serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct LdtkIntGridCell(i64);

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct LdtkEntity {
    pub grid: IVec2,
    pub identifier: String,
    pub pivot: Vec2,
    pub tile: Option<LdtkEntityTile>,
    pub def_uid: i64,
    pub field_instances: Vec<LdtkField>,
    pub height: i64,
    pub px: IVec2,
    pub width: i64,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct LdtkField {
    pub identifier: String,
    pub value: Option<Value>,
    pub def_uid: i64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct LdtkEntityTile {
    pub src_rect: Rect<i64>,
    pub tileset_uid: i64,
}

pub fn process_loaded_ldtk(
    mut commands: Commands,
    mut ldtk_events: EventReader<AssetEvent<LdtkAsset>>,
    mut ldtk_map_query: Query<(Entity, &Handle<LdtkAsset>, &LevelSelection, &mut Map)>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    new_ldtks: Query<&Handle<LdtkAsset>, Added<Handle<LdtkAsset>>>,
) {
    // This function uses code from the bevy_ecs_tilemap ldtk example
    // https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/ldtk/ldtk.rs
    let mut changed_ldtks = Vec::<Handle<LdtkAsset>>::new();
    for event in ldtk_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                info!("Ldtk added!");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                info!("Ldtk changed!");
                changed_ldtks.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                info!("Ldtk removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_ldtks = changed_ldtks
                    .into_iter()
                    .filter(|changed_handle| changed_handle == handle)
                    .collect();
            }
        }
    }

    for new_ldtk_handle in new_ldtks.iter() {
        changed_ldtks.push(new_ldtk_handle.clone());
    }

    for changed_ldtk in changed_ldtks.iter() {
        for (entity, ldtk_handle, level_selection, map) in ldtk_map_query
            .iter_mut()
            .filter(|(_, l, _, _)| changed_ldtk == *l)
        {
            //TODO: despawn changed levels

            let ldtk_asset = ldtk_assets.get(ldtk_handle).unwrap();
            for (_, level) in
                ldtk_asset.project.levels.iter().enumerate().filter(
                    |(i, l)| match level_selection {
                        LevelSelection::Identifier(s) => *s == l.identifier,
                        LevelSelection::Index(j) => j == i,
                        LevelSelection::Uid(u) => *u == l.uid,
                    },
                )
            {
                if let Some(layer_instances) = &level.layer_instances {
                    for layer_instance in layer_instances {
                        //TODO: spawn layers
                    }
                }
            }
        }
    }
}
