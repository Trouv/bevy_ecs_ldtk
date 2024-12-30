use quote::quote;
use syn::{Error, ExprClosure, LitInt, LitStr, Path, Token};

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
            .find(|a| *a.path().get_ident().as_ref().unwrap() == SPRITE_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite {
            match expand_sprite_attribute(attribute, field_name, field_type) {
                Ok(attr) => field_constructions.push(attr),
                Err(e) => return e.into(),
            }
            continue;
        }

        let sprite_sheet = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == SPRITE_SHEET_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite_sheet {
            match expand_sprite_sheet_attribute(attribute, field_name, field_type) {
                Ok(attr) => field_constructions.push(attr),
                Err(e) => return e.into(),
            }
            continue;
        }

        let worldly = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == WORLDLY_ATTRIBUTE_NAME);
        if let Some(attribute) = worldly {
            field_constructions.push(expand_worldly_attribute(attribute, field_name, field_type));
            continue;
        }

        let grid_coords = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == GRID_COORDS_ATTRIBUTE_NAME);
        if let Some(attribute) = grid_coords {
            field_constructions.push(expand_grid_coords_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let ldtk_entity = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == LDTK_ENTITY_ATTRIBUTE_NAME);
        if let Some(attribute) = ldtk_entity {
            field_constructions.push(expand_ldtk_entity_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let from_entity_instance = field.attrs.iter().find(|a| {
            *a.path().get_ident().as_ref().unwrap() == FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME
        });
        if let Some(attribute) = from_entity_instance {
            field_constructions.push(expand_from_entity_instance_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let with = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == WITH_ATTRIBUTE_NAME);
        if let Some(attribute) = with {
            match expand_with_attribute(attribute, field_name, field_type) {
                Ok(attr) => field_constructions.push(attr),
                Err(e) => return e.into(),
            }
            continue;
        }

        let default = field
            .attrs
            .iter()
            .find(|a| *a.path().get_ident().as_ref().unwrap() == DEFAULT_ATTRIBUTE_NAME);
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
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
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
        .meta
    {
        syn::Meta::List(syn::MetaList { .. }) => {
            let path = attribute.parse_args::<LitStr>()
                .map_err(|_| Error::new_spanned(
                    attribute,
                    "#[sprite...] attribute should take the form #[sprite(\"asset/path.png\")] or #[sprite]").into_compile_error()
                )?;                    let asset_path = &path.value();

                    Ok(quote! {
                        #field_name: bevy::prelude::Sprite::from_image(
                            asset_server.load(#asset_path)),
                    })
        },
        syn::Meta::Path(_) => {
            Ok(quote! {
                #field_name: bevy_ecs_ldtk::utils::sprite_from_entity_info(tileset),
            })
        },
        _ => panic!("#[sprite...] attribute should take the form #[sprite(\"asset/path.png\")] or #[sprite]"),
    }
}

#[derive(Clone, Debug)]
struct SpriteSheetAttributeLong {
    path: String,
    width: u32,
    height: u32,
    columns: u32,
    rows: u32,
    padding: u32,
    offset: u32,
    index: usize,
}

impl syn::parse::Parse for SpriteSheetAttributeLong {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<LitStr>().map_err(|e| {
            Error::new(
                e.span(),
                "First argument of #[sprite_sheet(...)] should be a string",
            )
        })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let width = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Second argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let height = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Third argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let columns = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Fourth argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let rows = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Fifth argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let padding = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Sixth argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let offset = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<u32>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Seventh argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        _ = input
            .parse::<Token![,]>()
            .map_err(|e| Error::new(e.span(), "Expected comma after first argument"))?;
        let index = input
            .parse::<LitInt>()
            .map(|int| int.base10_parse::<usize>().unwrap())
            .map_err(|e| {
                Error::new(
                    e.span(),
                    "Eighth argument of #[sprite_sheet(...)] should be an int",
                )
            })?;
        Ok(Self {
            path: path.value().to_string(),
            width,
            height,
            columns,
            rows,
            padding,
            offset,
            index,
        })
    }
}

fn expand_sprite_sheet_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
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
    .meta
    {
        syn::Meta::List(syn::MetaList { ref tokens,.. }) => {
            let s = tokens.to_string();
            if s == "no_grid" {
                return Ok(quote! {
                    #field_name: bevy_ecs_ldtk::utils::sprite_sheet_from_entity_info(entity_instance, tileset, tileset_definition, texture_atlases, false),
                })
            }

            if !s.contains(",") {
                // dirty hack
                return Err(Error::new_spanned(attribute, "#[sprite_sheet...] attribute should take the form #[sprite_sheet(\"asset/path.png\", tile_width, tile_height, columns, rows, padding, offset, index)], #[sprite_sheet(no_grid)] or #[sprite_sheet]").into_compile_error())
            }

            let attributes_args = syn::parse2::<SpriteSheetAttributeLong>(tokens.clone()).map_err(|e| e.into_compile_error())?;            let SpriteSheetAttributeLong { path: asset_path, width: tile_width, height: tile_height, columns, rows, padding, offset, index } = attributes_args;


            Ok(quote! {
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
            })
        },
        syn::Meta::Path(_) => {
            Ok(quote! {
                #field_name: bevy_ecs_ldtk::utils::sprite_sheet_from_entity_info(entity_instance, tileset, tileset_definition, texture_atlases, true),
            })
        },
        _ => panic!("#[sprite_sheet...] attribute should take the form #[sprite_sheet(\"asset/path.png\", tile_width, tile_height, columns, rows, padding, offset, index)], #[sprite_sheet(no_grid)] or #[sprite_sheet]"),
    }
}

fn expand_worldly_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute.meta {
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
    match attribute.meta {
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
    match attribute.meta {
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
    match attribute.meta {
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
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    if let syn::Meta::List(syn::MetaList { ref tokens, .. }) = attribute.meta {
        if let Ok(path) = syn::parse2::<Path>(tokens.clone()) {
            return Ok(quote! {
                #field_name: #path(entity_instance),
            });
        }
        if let Ok(closure) = syn::parse2::<ExprClosure>(tokens.clone()) {
            return Ok(quote! {
                #field_name: (#closure)(entity_instance),
            });
        }
    }
    Err(Error::new_spanned(
        attribute,
        "#[with...] attribute should take the form #[with(function_name) or #[with(|_| {..})]]",
    )
    .to_compile_error())
}

fn expand_default_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    _: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute.meta {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: Default::default(),
            }
        }
        _ => panic!("#[default] attribute should take the form #[default]"),
    }
}
