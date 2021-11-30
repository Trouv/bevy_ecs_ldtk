use proc_macro;
use proc_macro2;
use quote::quote;
use syn;

static LDTK_INT_CELL_ATTRIBUTE_NAME: &str = "ldtk_int_cell";
static FROM_INT_GRID_CELL_ATTRIBUTE_NAME: &str = "from_int_grid_cell";

pub fn expand_ldtk_int_cell_derive(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
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

        let ldtk_int_cell = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == LDTK_INT_CELL_ATTRIBUTE_NAME);
        if let Some(attribute) = ldtk_int_cell {
            field_constructions.push(expand_ldtk_int_cell_attribute(
                attribute, field_name, field_type,
            ));
            continue;
        }

        let from_int_grid_cell = field
            .attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == FROM_INT_GRID_CELL_ATTRIBUTE_NAME);
        if let Some(attribute) = from_int_grid_cell {
            field_constructions.push(expand_from_int_grid_cell_attribute(
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
        impl #impl_generics bevy_ecs_ldtk::prelude::LdtkIntCell for #struct_name #ty_generics #where_clause {
            fn bundle_entity(
                int_grid_cell: &bevy_ecs_ldtk::prelude::IntGridCell,
            ) -> Self {
                Self {
                    #(#field_constructions)*
                }
            }
        }
    };
    gen.into()
}

fn expand_ldtk_int_cell_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[ldtk_int_cell] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: #field_type::bundle_int_cell(int_grid_cell, tileset_map, asset_server, materials, texture_atlases),
            }
        }
        _ => panic!("#[ldtk_int_cell] attribute should take the form #[ldtk_int_cell]"),
    }
}

fn expand_from_int_grid_cell_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse #[from_int_grid_cell] attribute")
    {
        syn::Meta::Path(_) => {
            quote! {
                #field_name: #field_type::from(int_grid_cell),
            }
        }
        _ => {
            panic!("#[from_int_grid_cell] attribute should take the form #[from_int_grid_cell]")
        }
    }
}
