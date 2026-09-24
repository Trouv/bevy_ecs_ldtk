use super::LdtkSceneContext;
use bevy::{prelude::*, scene::Scene};

mod sprite;

pub trait LdtkSceneMarker: Component + Clone {
    fn scene(&self, ctx: &LdtkSceneContext, world: &mut World) -> impl Scene + use<Self>;
}
