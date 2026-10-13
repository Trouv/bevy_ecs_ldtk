use super::PhantomLdtkEntityTrait;
use crate::{
    app::EntitySceneFn,
    ldtk::{EntityInstance, LayerInstance, TilesetDefinition},
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::marker::PhantomData;

/// An entity instance from LDtk that has had references resolved for ease of use.
pub struct EntityInstanceContext<'a> {
    /// The raw entity instance as it exists in LDtk.
    pub entity_instance: &'a EntityInstance,
    /// A reference to the layer instance this entity instance is associated with.
    pub layer_instance: &'a LayerInstance,
    /// A handle for the tileset image if one can be resolved from the path on the
    /// tileset definition.
    pub tileset: Option<&'a Handle<Image>>,
    /// The definition of the entity's tileset from LDtk (if one is configured).
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
