use super::PhantomLdtkEntityTrait;
use crate::{
    components::UseLdtkSprite,
    ldtk::{EntityInstance, LayerInstance, TilesetDefinition},
};
use bevy::{ecs::system::EntityCommands, prelude::*, scene::Scene};

mod marker;
pub use marker::LdtkSceneMarker;

pub struct LdtkSceneContext {
    pub entity_instance: EntityInstance,
    pub tileset: Option<Handle<Image>>,
    pub tileset_definition: Option<TilesetDefinition>,
}

fn fill<M: LdtkSceneMarker>(entity: &mut EntityWorldMut, ctx: &LdtkSceneContext) {
    if let Some(marker) = entity.get::<M>().cloned() {
        let scene = entity.world_scope(|world| marker.scene(ctx, world));
        entity.remove::<M>();
        if let Err(err) = bevy::scene::EntityWorldMutSceneExt::apply_scene(entity, scene) {
            error!("failed to apply LDtk scene marker: {err}");
        }
    }
}

pub struct LdtkEntityScene<F>(pub F);

impl<S: Scene, F: Fn() -> S> PhantomLdtkEntityTrait for LdtkEntityScene<F> {
    fn evaluate<'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'a>,
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> &'b mut EntityCommands<'a> {
        let ctx = LdtkSceneContext {
            entity_instance: entity_instance.clone(),
            tileset: tileset.cloned(),
            tileset_definition: tileset_definition.cloned(),
        };

        bevy::scene::EntityCommandsSceneExt::apply_scene(entity_commands, (self.0)());

        entity_commands.queue(move |mut entity: EntityWorldMut| {
            fill::<UseLdtkSprite>(&mut entity, &ctx);
        })
    }
}
