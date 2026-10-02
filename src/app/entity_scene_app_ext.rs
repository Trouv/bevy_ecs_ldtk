use crate::app::ldtk_entity::{scene::LdtkEntityScene, *};
use bevy::{prelude::*, scene::Scene};
use std::marker::PhantomData;

pub use crate::app::ldtk_entity::scene::EntityInstanceContext;

pub trait EntitySceneFn<Marker>: 'static {
    fn call(&self, ctx: &EntityInstanceContext) -> impl Scene;
}

pub struct ContextualEntitySceneFn;

impl<S: Scene, F: Fn(&EntityInstanceContext) -> S + 'static> EntitySceneFn<ContextualEntitySceneFn>
    for F
{
    fn call(&self, ctx: &EntityInstanceContext) -> impl Scene {
        self(ctx)
    }
}

pub struct BareEntitySceneFn;

impl<S: Scene, F: Fn() -> S + 'static> EntitySceneFn<BareEntitySceneFn> for F {
    fn call(&self, _: &EntityInstanceContext) -> impl Scene {
        self()
    }
}

pub struct ComponentEntitySceneFn;

impl<C: Component + Clone + Default + Unpin> EntitySceneFn<ComponentEntitySceneFn> for C {
    fn call(&self, _: &EntityInstanceContext) -> impl Scene {
        bsn! { C }
    }
}

pub trait LdtkEntitySceneAppExt {
    fn register_ldtk_entity_scene_for_layer_optional<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
        scene: F,
    ) -> &mut Self;

    fn register_ldtk_entity_scene<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        entity_identifier: &str,
        scene: F,
    ) -> &mut Self {
        self.register_ldtk_entity_scene_for_layer_optional(
            None,
            Some(entity_identifier.to_string()),
            scene,
        )
    }
}

impl LdtkEntitySceneAppExt for App {
    fn register_ldtk_entity_scene_for_layer_optional<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
        scene: F,
    ) -> &mut Self {
        let new_entry = Box::new(LdtkEntityScene(scene, PhantomData));
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
