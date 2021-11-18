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
        }
        _ => panic!("#[sprite_bundle...] attribute should take the form #[sprite_bundle(\"asset/path.png\")] or #[sprite_bundle]"),
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
