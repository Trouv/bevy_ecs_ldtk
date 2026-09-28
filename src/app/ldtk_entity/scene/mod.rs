use super::PhantomLdtkEntityTrait;
use crate::{
    components::UseLdtkSprite,
    ldtk::{EntityInstance, LayerInstance, TilesetDefinition},
};
use bevy::{ecs::system::EntityCommands, prelude::*, scene::Scene};

pub struct LdtkSceneContext {
    pub entity_instance: EntityInstance,
    pub tileset: Option<Handle<Image>>,
    pub tileset_definition: Option<TilesetDefinition>,
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
    }
}
