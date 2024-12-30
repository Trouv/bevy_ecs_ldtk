use quote::quote;

static SPRITE_ATTRIBUTE_NAME: &str = "sprite";
static SPRITE_SHEET_ATTRIBUTE_NAME: &str = "sprite_sheet";
static WORLDLY_ATTRIBUTE_NAME: &str = "worldly";
static GRID_COORDS_ATTRIBUTE_NAME: &str = "grid_coords";
static LDTK_ENTITY_ATTRIBUTE_NAME: &str = "ldtk_entity";
static FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME: &str = "from_entity_instance";
static WITH_ATTRIBUTE_NAME: &str = "with";
static DEFAULT_ATTRIBUTE_NAME: &str = "default";

pub fn expand_ldtk_entity_derive(ast: syn::DeriveInput) -> proc_macro::TokenStream {
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

        let sprite = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == SPRITE_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite {
            field_constructions.push(expand_sprite_attribute(attribute, field_name, field_type));
            continue;
        }

        let sprite_sheet = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == SPRITE_SHEET_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite_sheet {
            field_constructions.push(expand_sprite_sheet_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let worldly = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == WORLDLY_ATTRIBUTE_NAME);
        if let Some(attribute) = worldly {
            field_constructions.push(expand_worldly_attribute(attribute, field_name, field_type));
            continue;
        }

        let grid_coords = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == GRID_COORDS_ATTRIBUTE_NAME);
        if let Some(attribute) = grid_coords {
            field_constructions.push(expand_grid_coords_attribute(
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

        let with = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == WITH_ATTRIBUTE_NAME);
        if let Some(attribute) = with {
            field_constructions.push(expand_with_attribute(attribute, field_name, field_type));
            continue;
        }

        let default = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == DEFAULT_ATTRIBUTE_NAME);
        if let Some(attribute) = default {
            field_constructions.push(expand_default_attribute(attribute, field_name, field_type));
            continue;
        }
    }

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let struct_update = if field_constructions.len() < fields.len() {
        quote! { ..<Self as std::default::Default>::default() }
    } else {
        quote! {}
    };

    let gen = quote! {
        impl #impl_generics bevy_ecs_ldtk::prelude::LdtkEntity for #struct_name #ty_generics #where_clause {
            fn bundle_entity(
                entity_instance: &bevy_ecs_ldtk::prelude::EntityInstance,
                layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
                tileset: Option<&bevy::prelude::Handle<bevy::prelude::Image>>,
                tileset_definition: Option<&bevy_ecs_ldtk::prelude::TilesetDefinition>,
                asset_server: &bevy::prelude::AssetServer,
                texture_atlases: &mut bevy::prelude::Assets<bevy::prelude::TextureAtlasLayout>,
            ) -> Self {
                Self {
                    #(#field_constructions)*
                    #struct_update
                }
            }
        }
    };
    gen.into()
}

fn expand_sprite_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    // check the type
    match field_type {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => {
            if let Some(last) = segments.last() {
                if last.ident != *"Sprite" {
                    panic!("#[sprite...] attribute should apply to a field of type bevy::prelude::Sprite")
                }
            }
        }
        _ => panic!("#[sprite...] attribute should apply to a field of type bevy::prelude::Sprite"),
    }

    match attribute
        .parse_meta()
        .expect("Cannot parse #[sprite...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
            match nested.first().unwrap() {
                syn::NestedMeta::Lit(syn::Lit::Str(asset)) => {
                    let asset_path = &asset.value();

                    quote! {
                        #field_name: bevy::prelude::Sprite::from_image(
                            asset_server.load(#asset_path)),
                    }
                },
                _ => panic!("Expected asset path as the only argument of #[sprite(...)]"),
            }
        },
        syn::Meta::Path(_) => {
            quote! {
                #field_name: bevy_ecs_ldtk::utils::sprite_from_entity_info(tileset),
            }
        },
        _ => panic!("#[sprite...] attribute should take the form #[sprite(\"asset/path.png\")] or #[sprite]"),
    }
}

fn expand_sprite_sheet_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    // check the type
    match field_type {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => {
            if let Some(last) = segments.last() {
                if last.ident != *"Sprite" {
                    panic!("#[sprite_sheet...] attribute should apply to a field of type bevy::prelude::Sprite")
                }
            }
        }
        _ => panic!(
            "#[sprite_sheet...] attribute should apply to a field of type bevy::prelude::Sprite"
        ),
    }

    match attribute
        .parse_meta()
        .expect("Cannot parse #[sprite_sheet...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 8 => {
            let mut nested_iter = nested.iter();

            let asset_path = &match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Str(asset))) => asset.value(),
                _ => panic!("First argument of #[sprite_sheet(...)] should be a string")
            };
            let tile_width = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Second argument of #[sprite_sheet(...)] should be an int")
            };
            let tile_height = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Third argument of #[sprite_sheet(...)] should be an int")
            };
            let columns = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Fourth argument of #[sprite_sheet(...)] should be an int")
            };
            let rows = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Fifth argument of #[sprite_sheet(...)] should be an int")
            };
            let padding = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Sixth argument of #[sprite_sheet(...)] should be an int")
            };
            let offset = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<u32>().unwrap(),
                _ => panic!("Seventh argument of #[sprite_sheet(...)] should be an int")
            };
            let index = match nested_iter.next() {
                Some(syn::NestedMeta::Lit(syn::Lit::Int(asset))) => asset.base10_parse::<usize>().unwrap(),
                _ => panic!("Eighth argument of #[sprite_sheet(...)] should be an int")
            };

            quote! {
                #field_name: bevy::prelude::Sprite::from_atlas_image(
                    asset_server.load(#asset_path),
                    bevy::prelude::TextureAtlas {
                        layout: texture_atlases.add(
                            bevy::prelude::TextureAtlasLayout::from_grid(
                                bevy::prelude::UVec2::new(#tile_width, #tile_height),
                                #columns, #rows, Some(bevy::prelude::UVec2::splat(#padding)),
                                Some(bevy::prelude::UVec2::splat(#offset)),
                            )),
                        index: #index
                    },
                ),
            }
        },
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
            let mut nested_iter = nested.iter();

            match nested_iter.next() {
                Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) if path.is_ident("no_grid") => {},
                _ => panic!("Argument of #[sprite_sheet(...)] should be no_grid")
            };

            quote! {
                #field_name: bevy_ecs_ldtk::utils::sprite_sheet_from_entity_info(entity_instance, tileset, tileset_definition, texture_atlases, false),
            }
        },
        syn::Meta::Path(_) => {
            quote! {
                #field_name: bevy_ecs_ldtk::utils::sprite_sheet_from_entity_info(entity_instance, tileset, tileset_definition, texture_atlases, true),
            }
        },
        _ => panic!("#[sprite_sheet...] attribute should take the form #[sprite_sheet(\"asset/path.png\", tile_width, tile_height, columns, rows, padding, offset, index)], #[sprite_sheet(no_grid)] or #[sprite_sheet]"),
    }
}

fn expand_worldly_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[worldly] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: bevy_ecs_ldtk::prelude::Worldly::from_entity_info(entity_instance),
            }
        }
        _ => panic!("#[worldly] attribute should take the form #[worldly]"),
    }
}

fn expand_grid_coords_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[grid_coords] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: bevy_ecs_ldtk::prelude::GridCoords::from_entity_info(entity_instance, layer_instance),
            }
        }
        _ => panic!("#[grid_coords] attribute should take the form #[grid_coords]"),
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
                #field_name: <#field_type as bevy_ecs_ldtk::prelude::LdtkEntity>::bundle_entity(entity_instance, layer_instance, tileset, tileset_definition, asset_server, texture_atlases),
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
                #field_name: <#field_type as From<&bevy_ecs_ldtk::prelude::EntityInstance>>::from(entity_instance),
            }
        }
        _ => {
            panic!("#[from_entity_instance] attribute should take the form #[from_entity_instance]")
        }
    }
}

fn expand_with_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[with...] attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) if nested.len() == 1 => {
            match nested.first().unwrap() {
                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                    quote! {
                        #field_name: #path(entity_instance),
                    }
                }
                _ => panic!("Expected function as the only argument of #[with(...)]"),
            }
        }
        _ => {
            panic!("#[with...] attribute should take the form #[with(function_name)]")
        }
    }
}

fn expand_default_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[default] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: Default::default(),
            }
        }
        _ => panic!("#[default] attribute should take the form #[default]"),
    }
}
