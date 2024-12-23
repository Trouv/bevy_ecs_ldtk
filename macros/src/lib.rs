use proc_macro::TokenStream;

mod ldtk_entity;
mod ldtk_int_cell;

#[proc_macro_derive(
    LdtkEntity,
    attributes(
        sprite,
        sprite_sheet,
        worldly,
        grid_coords,
        ldtk_entity,
        from_entity_instance,
        with,
        default,
    )
)]
pub fn ldtk_entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    ldtk_entity::expand_ldtk_entity_derive(ast)
}

#[proc_macro_derive(
    LdtkIntCell,
    attributes(ldtk_int_cell, from_int_grid_cell, with, default)
)]
pub fn ldtk_int_cell_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    ldtk_int_cell::expand_ldtk_int_cell_derive(ast)
}
