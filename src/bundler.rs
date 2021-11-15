use crate::ldtk::EntityInstance;
use bevy::prelude::*;

pub trait EntityInstanceBundler {
    type Bundle: Bundle;
    fn bundle(&self, entity_instance: EntityInstance) -> Self::Bundle;
}
