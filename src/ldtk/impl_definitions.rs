use crate::ldtk::{Definitions, Type};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

impl Definitions {
    /// Creates image that will be used for rendering IntGrid colors.
    ///
    /// The resulting image is completely white and can be thought of as a single tile of the grid.
    /// The image is only as big as the biggest int grid layer's grid size.
    ///
    /// Can return `None` if there are no IntGrid layers that will be rendered by color.
    /// IntGrid layers that have a tileset are excluded since they will not be rendered by color.
    pub fn create_int_grid_image(&self) -> Option<Image> {
        self.layers
            .iter()
            .filter(|l| l.purple_type == Type::IntGrid && l.tileset_def_uid.is_none())
            .max_by(|layer_a, layer_b| layer_a.grid_size.cmp(&layer_b.grid_size))
            .map(|l| {
                Image::new_fill(
                    Extent3d {
                        width: l.grid_size as u32,
                        height: l.grid_size as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &[255, 255, 255, 255],
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default(),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::ldtk::LayerDefinition;

    use super::*;

    #[test]
    fn int_grid_image_is_white() {
        let definitions = Definitions {
            layers: vec![LayerDefinition {
                purple_type: Type::IntGrid,
                grid_size: 16,
                ..default()
            }],
            ..default()
        };

        let image = definitions.create_int_grid_image().unwrap();

        for byte in image.data.iter() {
            assert_eq!(*byte, 255);
        }
    }

    #[test]
    fn int_grid_image_is_size_of_max_int_grid_layer() {
        let definitions = Definitions {
            layers: vec![
                LayerDefinition {
                    purple_type: Type::IntGrid,
                    grid_size: 16,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::IntGrid,
                    grid_size: 32,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::IntGrid,
                    grid_size: 2,
                    ..default()
                },
                // Excludes non-intgrid layers
                LayerDefinition {
                    purple_type: Type::AutoLayer,
                    grid_size: 64,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::Tiles,
                    grid_size: 64,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::Entities,
                    grid_size: 64,
                    ..default()
                },
                // Excludes intgrid layers w/ tileset
                LayerDefinition {
                    purple_type: Type::IntGrid,
                    grid_size: 64,
                    tileset_def_uid: Some(1),
                    ..default()
                },
            ],
            ..default()
        };

        let image = definitions.create_int_grid_image().unwrap();

        assert_eq!(image.size(), UVec2::splat(32));
    }

    #[test]
    fn no_int_grid_image_for_no_elligible_int_grid_layers() {
        let definitions = Definitions {
            layers: vec![
                // Excludes non-intgrid layers
                LayerDefinition {
                    purple_type: Type::AutoLayer,
                    grid_size: 64,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::Tiles,
                    grid_size: 64,
                    ..default()
                },
                LayerDefinition {
                    purple_type: Type::Entities,
                    grid_size: 64,
                    ..default()
                },
                // Excludes intgrid layers w/ tileset
                LayerDefinition {
                    purple_type: Type::IntGrid,
                    grid_size: 64,
                    tileset_def_uid: Some(1),
                    ..default()
                },
            ],
            ..default()
        };

        assert!(definitions.create_int_grid_image().is_none());
    }
}
