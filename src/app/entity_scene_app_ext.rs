//! Provides [LdtkEntitySceneAppExt] for registering scenes to spawn for given LDtk Entity identifiers.
use crate::app::ldtk_entity::{scene::LdtkEntityScene, *};
use bevy::prelude::*;

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

/// Provides functions for registering [Scene]s (or things can can be represented as a [Scene]) that
/// should be spawned for specific LDtk entities.
pub trait LdtkEntitySceneAppExt {
    /// Similar to [LdtkEntitySceneAppExt::register_ldtk_entity_scene], except that it can be scoped
    /// to only entities on a specific layer in LDtk.
    fn register_ldtk_entity_scene_for_layer<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        layer_identifier: &str,
        entity_identifier: &str,
        scene: F,
    ) -> &mut Self;

    /// Register a [EntitySceneFn] that the plugin should spawn for LDtk entities that match the given
    /// LDtk entity identifier.
    ///
    /// [EntitySceneFn] has a few builtin implemenations, shown belown.
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// fn main() {
    ///     App::empty()
    ///         .add_plugins(LdtkPlugin)
    ///         // Register a scene-compatible component.
    ///         .register_ldtk_entity_scene("component_entity", ComponentA)
    ///         // Register a function that returns a scene.
    ///         .register_ldtk_entity_scene("scene_entity", scene)
    ///         // Register a scene function with access to the full context of the
    ///         // entity instance as it lives in LDtk.
    ///         .register_ldtk_entity_scene("custom_scene_entity", scene_with_context)
    ///         .run();
    /// }
    ///
    /// #[derive(Component, Default, Clone)]
    /// struct ComponentA;
    ///
    /// fn scene() -> impl Scene {
    ///     bsn! { ComponentA }
    /// }
    ///
    /// fn scene_with_context(ctx: &EntityInstanceContext) -> impl Scene {
    ///     let name = ctx.entity_instance.identifier.clone();
    ///     bsn! {
    ///         Name(name)
    ///         ComponentA
    ///     }
    /// }
    /// ```
    fn register_ldtk_entity_scene<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        entity_identifier: &str,
        scene: F,
    ) -> &mut Self;
}

impl LdtkEntitySceneAppExt for App {
    fn register_ldtk_entity_scene_for_layer<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        layer_identifier: &str,
        entity_identifier: &str,
        scene: F,
    ) -> &mut Self {
        insert_scene(self, Some(layer_identifier), entity_identifier, scene);
        self
    }

    fn register_ldtk_entity_scene<M: 'static, F: EntitySceneFn<M>>(
        &mut self,
        entity_identifier: &str,
        scene: F,
    ) -> &mut Self {
        insert_scene(self, None, entity_identifier, scene);
        self
    }
}

fn insert_scene<M: 'static, F: EntitySceneFn<M>>(
    app: &mut App,
    layer_identifier: Option<&str>,
    entity_identifier: &str,
    scene: F,
) {
    let key = (
        layer_identifier.map(str::to_owned),
        Some(entity_identifier.to_owned()),
    );
    let new_entry = Box::new(LdtkEntityScene::new(scene));
    match app.world_mut().get_non_send_mut::<LdtkEntityMap>() {
        Some(mut entries) => {
            entries.insert(key, new_entry);
        }
        None => {
            let mut scene_map = LdtkEntityMap::new();
            scene_map.insert(key, new_entry);
            app.world_mut().insert_non_send::<LdtkEntityMap>(scene_map);
        }
    }
}
