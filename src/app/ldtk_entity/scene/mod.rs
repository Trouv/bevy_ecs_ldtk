use super::PhantomLdtkEntityTrait;
use crate::{
    app::EntitySceneFn,
    ldtk::{EntityInstance, LayerInstance, TilesetDefinition},
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::marker::PhantomData;

pub struct EntityInstanceContext<'a> {
    pub entity_instance: &'a EntityInstance,
    pub layer_instance: &'a LayerInstance,
    pub tileset: Option<&'a Handle<Image>>,
    pub tileset_definition: Option<&'a TilesetDefinition>,
}

pub(crate) struct LdtkEntityScene<F, M>(F, PhantomData<M>);

impl<F, M> LdtkEntityScene<F, M> {
    pub(crate) fn new(scene: F) -> Self {
        Self(scene, PhantomData)
    }
}

impl<M: 'static, F: EntitySceneFn<M>> PhantomLdtkEntityTrait for LdtkEntityScene<F, M> {
    fn evaluate<'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'a>,
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> &'b mut EntityCommands<'a> {
        let ctx = EntityInstanceContext {
            entity_instance,
            layer_instance,
            tileset,
            tileset_definition,
        };

        EntityCommandsSceneExt::apply_scene(entity_commands, self.0.call(&ctx))
    }
}
