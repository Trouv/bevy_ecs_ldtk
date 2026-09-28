use super::PhantomLdtkEntityTrait;
use crate::ldtk::{EntityInstance, LayerInstance, TilesetDefinition};
use bevy::{ecs::system::EntityCommands, prelude::*, scene::Scene};

pub struct EntityInstanceContext<'a> {
    pub entity_instance: &'a EntityInstance,
    pub layer_instance: &'a LayerInstance,
    pub tileset: Option<&'a Handle<Image>>,
    pub tileset_definition: Option<&'a TilesetDefinition>,
}

pub struct LdtkEntityScene<F>(pub F);

impl<S: Scene, F: Fn(&EntityInstanceContext) -> S> PhantomLdtkEntityTrait for LdtkEntityScene<F> {
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

        bevy::scene::EntityCommandsSceneExt::apply_scene(entity_commands, (self.0)(&ctx))
    }
}
