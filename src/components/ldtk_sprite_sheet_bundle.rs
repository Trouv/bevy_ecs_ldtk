use bevy::{
    asset::{AssetServer, Assets, Handle},
    prelude::Bundle,
    render::texture::Image,
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
};

use crate::{
    prelude::{LayerInstance, LdtkEntity, TilesetDefinition},
    utils, EntityInstance,
};

/// [`Bundle`] for sprite-sheet-based sprites, similar to bevy 0.13's `SpriteSheetBundle`.
///
/// Implements [`LdtkEntity`], and can be added to an [`LdtkEntity`] bundle with the `#[sprite_sheet_bundle]`
/// field attribute.
/// See [`LdtkEntity#sprite_sheet_bundle`] for attribute macro usage.
///
/// [`Bundle`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.Bundle.html
#[derive(Bundle, Clone, Debug, Default)]
pub struct LdtkSpriteSheetBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
}

impl LdtkEntity for LdtkSpriteSheetBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        utils::sprite_sheet_bundle_from_entity_info(
            entity_instance,
            tileset,
            tileset_definition,
            texture_atlases,
            true,
        )
    }
}
