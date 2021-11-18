use proc_macro2::TokenStream;
use quote::quote;
use syn;

pub fn expand_sprite_bundle_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> TokenStream {
    // check the type
    match field_type {
        syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) => {
            if let Some(last) = segments.last() {
                if last.ident.to_string() != "SpriteBundle".to_string() {
                    panic!("#[sprite_bundle...] attribute should apply to a field of type bevy::prelude::SpriteBundle")
                }
            }
        },
        _ => panic!("#[sprite_bundle...] attribute should apply to a field of type bevy::prelude::SpriteBundle")
    }

    match attribute
        .parse_meta()
        .expect("Cannot parse #[sprite_bundle...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
            match nested.first().unwrap() {
                syn::NestedMeta::Lit(syn::Lit::Str(asset)) => {
                    let asset_path = &asset.value();

                    quote! {
                        #field_name: bevy::prelude::SpriteBundle {
                            material: materials.add(asset_server.load(#asset_path).into()),
                            ..Default::default()
                        },
                    }
                },
                _ => panic!("Expected asset path as the only argument of #[sprite_bundle(...)]"),
            }
        },
        syn::Meta::Path(_) => {
            quote! {
            #field_name: bevy::prelude::SpriteBundle {
                    material: materials.add(
                            tileset_map.get(&entity_instance
                                .tile.clone()
                                .expect("#[sprite_bundle] attribute expected the EntityInstance to have a tile defined.")
                                .tileset_uid
                            ).expect("EntityInstance's tileset should be in the tileset_map").clone().into()
                        ),
                    ..Default::default()
                },
            }
        },
        _ => panic!("#[sprite_bundle...] attribute should take the form #[sprite_bundle(\"asset/path.png\")] or #[sprite_bundle]"),
    }
}

pub fn expand_sprite_sheet_bundle_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> TokenStream {
    // check the type
    match field_type {
        syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) => {
            if let Some(last) = segments.last() {
                if last.ident.to_string() != "SpriteSheetBundle".to_string() {
                    panic!("#[sprite_sheet_bundle...] attribute should apply to a field of type bevy::prelude::SpriteSheetBundle")
                }
            }
        },
        _ => panic!("#[sprite_sheet_bundle...] attribute should apply to a field of type bevy::prelude::SpriteSheetBundle")
    }

    match attribute
        .parse_meta()
        .expect("Cannot parse #[sprite_sheet_bundle...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 6 => {
            let mut nested_iter = nested.iter();

            let asset_path = &match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Str(asset))) => asset.value(),
                _ => panic!("First argument of #[sprite_sheet_bundle(...)] should be a string")
            };
            let tile_width = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Float(asset))) => asset.base10_digits(),
                _ => panic!("Second argument of #[sprite_sheet_bundle(...)] should be a float")
            };
            let tile_height = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Float(asset))) => asset.base10_digits(),
                _ => panic!("Third argument of #[sprite_sheet_bundle(...)] should be a float")
            };
            let num_columns = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_digits(),
                _ => panic!("Fourth argument of #[sprite_sheet_bundle(...)] should be an int")
            };
            let num_rows = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_digits(),
                _ => panic!("Fifth argument of #[sprite_sheet_bundle(...)] should be an int")
            };
            let index = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_digits(),
                _ => panic!("Sixth argument of #[sprite_sheet_bundle(...)] should be an int")
            };


            quote! {
                #field_name: bevy::prelude::SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(
                        bevy::prelude::TextureAtlas::from_grid(
                            asset_server.load(#asset_path).into(),
                            bevy::prelude::Vec2::new(#tile_width, #tile_height),
                            #num_rows, #num_columns,
                        )
                    ),
                    texture_atlas_sprite: bevy::prelude::TextureAtlasSprite {
                        index: #index,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        },
        _ => panic!("#[sprite_sheet_bundle...] attribute should take the form #[sprite_sheet_bundle(\"asset/path.png\", tile_width, tile_height, num_columns, num_rows, index)]"),
    }
}

pub fn expand_entity_instance_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> TokenStream {
    match field_type {
        syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) => {
            if let Some(last) = segments.last() {
                if last.ident.to_string() != "EntityInstance".to_string() {
                    panic!("#[entity_instance] attribute should apply to a field of type bevy_ecs_ldtk::prelude::EntityInstance")
                }
            }
        },
        _ => panic!("#[entity_instance] attribute should apply to a field of type bevy_ecs_ldtk::prelude::EntityInstance")
    }

    match attribute
        .parse_meta()
        .expect("Cannot parse #[entity_instance] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: entity_instance.clone(),
            }
        }
        _ => panic!("#[entity_instance] attribute should take the form #[entity_instance]"),
    }
}
