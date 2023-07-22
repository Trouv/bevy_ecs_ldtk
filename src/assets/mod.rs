//! Assets and AssetLoaders for loading ldtk files.

use crate::ldtk;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::BoxedFuture,
};
use std::path::Path;

mod ldtk_project;
pub use ldtk_project::{LdtkLoader, LdtkProject};

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

impl ldtk::Definitions {
    /// Creates image that will be used for rendering IntGrid colors.
    ///
    /// The resulting image is completely white and can be thought of as a single tile of the grid.
    /// The image is only as big as the biggest int grid layer's grid size.
    ///
    /// Can return `None` if there are no IntGrid layers that will be rendered by color.
    /// IntGrid layers that have a tileset are excluded since they will not be rendered by color.
    fn create_int_grid_image(&self) -> Option<Image> {
        self.layers
            .iter()
            .filter(|l| l.purple_type == ldtk::Type::IntGrid && l.tileset_def_uid.is_none())
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
                )
            })
    }
}

impl ldtk::LdtkJson {
    /// Used for [LdtkProject::iter_levels].
    pub fn iter_levels(&self) -> impl Iterator<Item = &ldtk::Level> {
        self.levels
            .iter()
            .chain(self.worlds.iter().flat_map(|w| &w.levels))
    }
}

/// Secondary asset for loading ldtk files, specific to level data.
///
/// Loaded as a labeled asset when loading a standalone ldtk file with [LdtkProject].
/// The label is just the level's identifier.
///
/// Loaded as a dependency to the [LdtkProject] when loading an ldtk file with external levels.
#[derive(TypeUuid, Reflect, FromReflect)]
#[uuid = "5448469b-2134-44f5-a86c-a7b829f70a0c"]
pub struct LdtkLevel {
    pub level: ldtk::Level,
    pub background_image: Option<Handle<Image>>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let level: ldtk::Level = serde_json::from_slice(bytes)?;

            let mut background_asset_path = None;
            let mut background_image = None;
            if let Some(rel_path) = &level.bg_rel_path {
                let asset_path =
                    ldtk_path_to_asset_path(load_context.path().parent().unwrap(), rel_path);
                background_asset_path = Some(asset_path.clone());
                background_image = Some(load_context.get_handle(asset_path));
            }

            let ldtk_level = LdtkLevel {
                level,
                background_image,
            };

            let mut loaded_asset = LoadedAsset::new(ldtk_level);

            if let Some(asset_path) = background_asset_path {
                loaded_asset = loaded_asset.with_dependency(asset_path);
            }

            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}

#[cfg(test)]
mod tests {
    use crate::ldtk::{Definitions, LayerDefinition, Type};

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

        assert_eq!(image.size(), Vec2::splat(32.));
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
