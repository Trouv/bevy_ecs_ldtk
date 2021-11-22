extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

mod attributes;

#[proc_macro_derive(
    LdtkEntity,
    attributes(
        sprite_bundle,
        entity_instance,
        sprite_sheet_bundle,
        ldtk_entity,
        from_entity_instance
    )
)]
pub fn ldtk_entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    expand_ldtk_entity_derive(&ast)
}

static SPRITE_BUNDLE_ATTRIBUTE_NAME: &str = "sprite_bundle";
static SPRITE_SHEET_BUNDLE_ATTRIBUTE_NAME: &str = "sprite_sheet_bundle";
static ENTITY_INSTANCE_ATTRIBUTE_NAME: &str = "entity_instance";
static LDTK_ENTITY_ATTRIBUTE_NAME: &str = "ldtk_entity";
static FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME: &str = "from_entity_instance";

fn expand_ldtk_entity_derive(ast: &syn::DeriveInput) -> TokenStream {
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
            field_constructions.push(attributes::expand_sprite_bundle_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let sprite_sheet_bundle = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == SPRITE_SHEET_BUNDLE_ATTRIBUTE_NAME);
        if let Some(attribute) = sprite_sheet_bundle {
            field_constructions.push(attributes::expand_sprite_sheet_bundle_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let entity_instance = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == ENTITY_INSTANCE_ATTRIBUTE_NAME);
        if let Some(attribute) = entity_instance {
            field_constructions.push(attributes::expand_entity_instance_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let ldtk_entity = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == LDTK_ENTITY_ATTRIBUTE_NAME);
        if let Some(attribute) = ldtk_entity {
            field_constructions.push(attributes::expand_ldtk_entity_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let from_entity_instance = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == FROM_ENTITY_INSTANCE_ATTRIBUTE_NAME);
        if let Some(attribute) = from_entity_instance {
            field_constructions.push(attributes::expand_from_entity_instance_attribute(
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
            fn from_instance(
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
