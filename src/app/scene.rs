use crate::ldtk::{EntityInstance, TilesetDefinition};
use bevy::{prelude::*, scene::Scene};

pub struct LdtkSceneContext {
    pub entity_instance: EntityInstance,
    pub tileset: Option<Handle<Image>>,
    pub tileset_definition: Option<TilesetDefinition>,
}

pub trait LdtkSceneMarker: Component + Clone {
    fn scene(&self, ctx: &LdtkSceneContext, world: &mut World) -> impl Scene + use<Self>;
}
