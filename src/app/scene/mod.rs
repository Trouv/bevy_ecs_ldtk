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

mod marker;
pub use marker::*;

pub(super) fn fill<M: LdtkSceneMarker>(entity: &mut EntityWorldMut, ctx: &LdtkSceneContext) {
    if let Some(marker) = entity.get::<M>().cloned() {
        let scene = entity.world_scope(|world| marker.scene(ctx, world));
        entity.remove::<M>();
        if let Err(err) = bevy::scene::EntityWorldMutSceneExt::apply_scene(entity, scene) {
            error!("failed to apply LDtk scene marker: {err}");
        }
    }
}
