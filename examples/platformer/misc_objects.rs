use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::colliders::ColliderBundle;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
}

pub struct MiscObjectsPlugin;

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ChestBundle>("Chest")
            .register_ldtk_entity::<PumpkinsBundle>("Pumpkins");
    }
}
