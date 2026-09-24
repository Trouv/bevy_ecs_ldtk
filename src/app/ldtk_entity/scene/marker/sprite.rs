use super::LdtkSceneMarker;
use crate::{app::scene::LdtkSceneContext, components::UseLdtkSprite, utils};
use bevy::{ecs::template::OptionTemplate, image::TextureAtlasTemplate, prelude::*, scene::Scene};

impl LdtkSceneMarker for UseLdtkSprite {
    fn scene(&self, ctx: &LdtkSceneContext, world: &mut World) -> impl Scene + use<> {
        let sprite = match self {
            UseLdtkSprite::Image => utils::sprite_from_entity_info(ctx.tileset.as_ref()),
            UseLdtkSprite::Sheet | UseLdtkSprite::SheetNoGrid => {
                world.resource_scope(|_, mut texture_atlases| {
                    utils::sprite_sheet_from_entity_info(
                        &ctx.entity_instance,
                        ctx.tileset.as_ref(),
                        ctx.tileset_definition.as_ref(),
                        &mut texture_atlases,
                        *self == UseLdtkSprite::Sheet,
                    )
                })
            }
        };

        let texture_atlas: OptionTemplate<TextureAtlasTemplate> = sprite
            .texture_atlas
            .map(|atlas| TextureAtlasTemplate {
                layout: atlas.layout.into(),
                index: atlas.index,
            })
            .into();

        bsn! {
            Sprite {
                image: {sprite.image},
                texture_atlas: {texture_atlas},
            }
        }
    }
}
