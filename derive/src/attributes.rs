use proc_macro2::TokenStream;
use quote::quote;
use syn;

pub fn quote_sprite_bundle_attribute(
    attribute: &syn::Attribute,
    field_name: &syn::Ident,
) -> TokenStream {
    match attribute
        .parse_meta()
        .expect("Cannot parse sprite_bundle attribute")
    {
        syn::Meta::List(syn::MetaList { nested, .. }) => match nested.first().unwrap() {
            syn::NestedMeta::Lit(syn::Lit::Str(asset)) => {
                let asset_path = &asset.value();

                quote! {
                    #field_name: SpriteBundle {
                        material: materials.add(asset_server.load(#asset_path).into()),
                        ..Default::default()
                    },
                }
            }
            _ => panic!("Expected asset path as the first argument of sprite_bundle"),
        },
        _ => panic!("Expected arguments with sprite_bundle attribute"),
    }
}
