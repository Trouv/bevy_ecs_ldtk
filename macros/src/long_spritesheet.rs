pub struct SpriteSheetAttributeLong {
    pub path: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub padding: u32,
    pub offset: u32,
    pub index: usize,
}

impl syn::parse::Parse for SpriteSheetAttributeLong {
    fn parse(mut input: syn::parse::ParseStream) -> syn::Result<Self> {
        let comma = |input: &mut syn::parse::ParseStream, nth: &str| {
            input
                .parse::<syn::Token![,]>()
                .unwrap_or_else(|_| panic!("Expected comma after {nth} argument"))
        };
        let path = input
            .parse::<syn::LitStr>()
            .expect("First argument of #[sprite_sheet(...)] should be a string")
            .value();
        comma(&mut input, "first");
        let tile_width = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Second argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "second");
        let tile_height = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Third argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "third");
        let columns = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Fourth argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "fourth");
        let rows = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Fifth argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "fifth");
        let padding = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Sixth argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "sixth");
        let offset = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Seventh argument of #[sprite_sheet(...)] should be an int (u32)");
        comma(&mut input, "seventh");
        let index = input
            .parse::<syn::LitInt>()
            .and_then(|int| int.base10_parse())
            .expect("Seventh argument of #[sprite_sheet(...)] should be an int (usize)");
        Ok(SpriteSheetAttributeLong {
            path,
            tile_width,
            tile_height,
            columns,
            rows,
            padding,
            offset,
            index,
        })
    }
}
