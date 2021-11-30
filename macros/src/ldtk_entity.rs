use proc_macro;
use proc_macro2;
use quote::quote;
use syn;

static SPRITE_BUNDLE_ATTRIBUTE_NAME: &str = "sprite_bundle";
static SPRITE_SHEET_BUNDLE_ATTRIBUTE_NAME: &str = "sprite_sheet_bundle";
static LDTK_ENTITY_ATTRIBUTE_NAME: &str = "ldtk_entity";
static FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME: &str = "from_entity_instance";

pub fn expand_ldtk_entity_derive(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields."),
    };

    let mut field_constructions = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let sprite_bundle = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == SPRITE_BUNDLE_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite_bundle {
            field_constructions.push(expand_sprite_bundle_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let sprite_sheet_bundle = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == SPRITE_SHEET_BUNDLE_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite_sheet_bundle {
            field_constructions.push(expand_sprite_sheet_bundle_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let ldtk_entity = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == LDTK_ENTITY_ATTRIBUTE_NAME);
        if let Some(attribute) = ldtk_entity {
            field_constructions.push(expand_ldtk_entity_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let from_entity_instance = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME);
        if let Some(attribute) = from_entity_instance {
            field_constructions.push(expand_from_entity_instance_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        field_constructions.push(quote! {
            #field_name: #field_type::default(),
        });
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics bevy_ecs_ldtk::prelude::LdtkEntity for #struct_name #ty_generics #where_clause {
            fn bundle_entity(
                entity_instance: &bevy_ecs_ldtk::prelude::EntityInstance,
                tileset_map: &bevy_ecs_ldtk::prelude::TilesetMap,
                asset_server: &bevy::prelude::AssetServer,
                materials: &mut bevy::prelude::Assets<bevy::prelude::ColorMaterial>,
                texture_atlases: &mut bevy::prelude::Assets<bevy::prelude::TextureAtlas>,
            ) -> Self {
                Self {
                    #(#field_constructions)*
                }
            }
        }
    };
    gen.into()
}

fn expand_sprite_bundle_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
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
                #field_name: #field_type::bundle_entity(entity_instance, tileset_map, asset_server, materials, texture_atlases),
            }
        },
        _ => panic!("#[sprite_bundle...] attribute should take the form #[sprite_bundle(\"asset/path.png\")] or #[sprite_bundle]"),
    }
}

fn expand_sprite_sheet_bundle_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
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
                Some(syn::NestedMeta::Lit(syn::Lit::Float(asset))) => asset.base10_parse::<f32>().unwrap(),
                _ => panic!("Second argument of #[sprite_sheet_bundle(...)] should be a float")
            };
            let tile_height = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Float(asset))) => asset.base10_parse::<f32>().unwrap(),
                _ => panic!("Third argument of #[sprite_sheet_bundle(...)] should be a float")
            };
            let columns = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<usize>().unwrap(),
                _ => panic!("Fourth argument of #[sprite_sheet_bundle(...)] should be an int")
            };
            let rows = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<usize>().unwrap(),
                _ => panic!("Fifth argument of #[sprite_sheet_bundle(...)] should be an int")
            };
            let index = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Sixth argument of #[sprite_sheet_bundle(...)] should be an int")
            };

            quote! {
                #field_name: bevy::prelude::SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(
                        bevy::prelude::TextureAtlas::from_grid(
                            asset_server.load(#asset_path).into(),
                            bevy::prelude::Vec2::new(#tile_width, #tile_height),
                            #columns, #rows,
                        )
                    ),
                    sprite: bevy::prelude::TextureAtlasSprite {
                        index: #index,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }
        },
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 2 => {
            let mut nested_iter = nested.iter();

            let columns = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<usize>().unwrap(),
                _ => panic!("First argument of #[sprite_sheet_bundle(columns, rows)] should be an int")
            };
            let rows = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<usize>().unwrap(),
                _ => panic!("Second argument of #[sprite_sheet_bundle(columns, rows)] should be an int")
            };

            quote! {
                #field_name: {
                    match entity_instance.tile.as_ref() {
                        Some(tile) => match tileset_map.get(&tile.tileset_uid) {
                            Some(tileset) => bevy::prelude::SpriteSheetBundle {
                                    texture_atlas: texture_atlases.add(
                                        bevy::prelude::TextureAtlas::from_grid(
                                            tileset.clone(),
                                            bevy::prelude::Vec2::new(
                                                tile.src_rect[2] as f32,
                                                tile.src_rect[3] as f32,
                                            ),
                                            #columns, #rows,
                                        )
                                    ),
                                    sprite: bevy::prelude::TextureAtlasSprite {
                                        index: (tile.src_rect[1] / tile.src_rect[3]) as u32
                                                * #columns as u32
                                                + (tile.src_rect[0] / tile.src_rect[2]) as u32,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                None => {
                                    warn!("EntityInstance's tileset should be in the TilesetMap");
                                    bevy::prelude::SpriteSheetBundle::default()
                                }
                            }
                        None => {
                            warn!("#[sprite_sheet_bundle(columns, rows)] attribute expected EntityInstance to have a tile defined");
                            bevy::prelude::SpriteSheetBundle::default()
                        }
                    }
                },
            }
        },
        _ => panic!("#[sprite_sheet_bundle...] attribute should take the form #[sprite_sheet_bundle(\"asset/path.png\", tile_width, tile_height, columns, rows, index)] or #[sprite_sheet_bundle(columns, rows)]"),
    }
}

fn expand_ldtk_entity_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[ldtk_entity] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: #field_type::bundle_entity(entity_instance, tileset_map, asset_server, materials, texture_atlases),
            }
        }
        _ => panic!("#[ldtk_entity] attribute should take the form #[ldtk_entity]"),
    }
}

fn expand_from_entity_instance_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[from_entity_instance] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: #field_type::from(entity_instance.clone()),
            }
        }
        _ => {
            panic!("#[from_entity_instance] attribute should take the form #[from_entity_instance]")
        }
    }
}
