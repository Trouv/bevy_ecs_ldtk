use proc_macro::TokenStream;
use syn;

mod ldtk_entity;

#[proc_macro_derive(
    LdtkEntity,
    attributes(sprite_bundle, sprite_sheet_bundle, ldtk_entity, from_entity_instance)
)]
pub fn ldtk_entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    ldtk_entity::expand_ldtk_entity_derive(&ast)
}
