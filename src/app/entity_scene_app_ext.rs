use crate::app::ldtk_entity::*;
use bevy::{prelude::*, scene::Scene};

pub trait LdtkEntitySceneAppExt {
    fn register_ldtk_entity_scene_for_layer_optional<S, F>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
        scene: F,
    ) -> &mut Self
    where
        S: Scene,
        F: Fn() -> S + 'static;

    fn register_ldtk_entity_scene<S, F>(&mut self, entity_identifier: &str, scene: F) -> &mut Self
    where
        S: Scene,
        F: Fn() -> S + 'static,
    {
        self.register_ldtk_entity_scene_for_layer_optional(
            None,
            Some(entity_identifier.to_string()),
            scene,
        )
    }
}

impl LdtkEntitySceneAppExt for App {
    fn register_ldtk_entity_scene_for_layer_optional<S, F>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
        scene: F,
    ) -> &mut Self
    where
        S: Scene,
        F: Fn() -> S + 'static,
    {
        let new_entry = Box::new(LdtkEntityScene(scene));
        match self.world_mut().get_non_send_mut::<LdtkEntityMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, entity_identifier), new_entry);
            }
            None => {
                let mut scene_map = LdtkEntityMap::new();
                scene_map.insert((layer_identifier, entity_identifier), new_entry);
                self.world_mut().insert_non_send::<LdtkEntityMap>(scene_map);
            }
        }
        self
    }
}
