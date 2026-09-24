use bevy::{ecs::VariantDefaults, prelude::*};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component, Reflect, VariantDefaults)]
#[reflect(Component)]
pub enum UseLdtkSprite {
    #[default]
    Sheet,
    SheetNoGrid,
    Image,
}
