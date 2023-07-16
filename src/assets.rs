//! Assets and AssetLoaders for loading ldtk files.

use crate::{ldtk, resources::LevelSelection};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::BoxedFuture,
};
use std::{collections::HashMap, path::Path};

#[allow(unused_imports)]
use crate::components::LdtkWorldBundle;

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

/// Used in [LdtkAsset]. Key is the tileset definition uid.
pub type TilesetMap = HashMap<i32, Handle<Image>>;

/// Used in [LdtkAsset]. Key is the level iid.
pub type LevelMap = HashMap<String, Handle<LdtkLevel>>;

/// Main asset for loading ldtk files.
///
/// Load your ldtk project with the asset server, then insert the handle into the
/// [LdtkWorldBundle].
#[derive(Clone, TypeUuid)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkAsset {
    pub project: ldtk::LdtkJson,
    pub tileset_map: TilesetMap,
    pub level_map: LevelMap,
    /// Image used for rendering int grid colors.
    pub int_grid_image_handle: Option<Handle<Image>>,
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
    /// Used for [LdtkAsset::iter_levels].
    pub fn iter_levels(&self) -> impl Iterator<Item = &ldtk::Level> {
        self.levels
            .iter()
            .chain(self.worlds.iter().flat_map(|w| &w.levels))
    }
}

impl LdtkAsset {
    pub fn world_height(&self) -> i32 {
        let mut world_height = 0;
        for level in self.iter_levels() {
            world_height = world_height.max(level.world_y + level.px_hei);
        }

        world_height
    }

    /// Get an iterator of all the levels in the LDtk file.
    ///
    /// This abstraction avoids compatibility issues between pre-multi-world and post-multi-world
    /// LDtk projects.
    ///
    /// Note: the returned levels are the ones existent in the [LdtkAsset].
    /// These levels will have "incomplete" data if you use LDtk's external levels feature.
    /// To always get full level data, you'll need to access `Assets<LdtkLevel>`.
    pub fn iter_levels(&self) -> impl Iterator<Item = &ldtk::Level> {
        self.project.iter_levels()
    }

    /// Find a particular level using a [LevelSelection].
    ///
    /// Note: the returned level is the one existent in the [LdtkAsset].
    /// This level will have "incomplete" data if you use LDtk's external levels feature.
    /// To always get full level data, you'll need to access `Assets<LdtkLevel>`.
    pub fn get_level(&self, level_selection: &LevelSelection) -> Option<&ldtk::Level> {
        self.iter_levels()
            .enumerate()
            .find(|(i, l)| level_selection.is_match(i, l))
            .map(|(_, l)| l)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkLoader;

impl AssetLoader for LdtkLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let project: ldtk::LdtkJson = serde_json::from_slice(bytes)?;

            let mut external_level_paths = Vec::new();
            let mut level_map = HashMap::new();
            let mut background_images = Vec::new();
            if project.external_levels {
                for level in project.iter_levels() {
                    if let Some(external_rel_path) = &level.external_rel_path {
                        let asset_path =
                            ldtk_path_to_asset_path(load_context.path(), external_rel_path);

                        external_level_paths.push(asset_path.clone());
                        level_map.insert(level.iid.clone(), load_context.get_handle(asset_path));
                    }
                }
            } else {
                for level in project.iter_levels() {
                    let label = level.identifier.as_ref();

                    let mut background_image = None;
                    if let Some(rel_path) = &level.bg_rel_path {
                        let asset_path = ldtk_path_to_asset_path(load_context.path(), rel_path);
                        background_images.push(asset_path.clone());
                        background_image = Some(load_context.get_handle(asset_path));
                    }

                    let ldtk_level = LdtkLevel {
                        level: level.clone(),
                        background_image,
                    };
                    let level_handle =
                        load_context.set_labeled_asset(label, LoadedAsset::new(ldtk_level));

                    level_map.insert(level.iid.clone(), level_handle);
                }
            }

            let mut tileset_rel_paths = Vec::new();
            let mut tileset_map = HashMap::new();
            for tileset in &project.defs.tilesets {
                if let Some(tileset_path) = &tileset.rel_path {
                    let asset_path = ldtk_path_to_asset_path(load_context.path(), tileset_path);

                    tileset_rel_paths.push(asset_path.clone());
                    tileset_map.insert(tileset.uid, load_context.get_handle(asset_path));
                } else if tileset.embed_atlas.is_some() {
                    warn!("Ignoring LDtk's Internal_Icons. They cannot be displayed due to their license.");
                } else {
                    let identifier = &tileset.identifier;
                    warn!("{identifier} tileset cannot be loaded, it has a null relative path.");
                }
            }

            let int_grid_image_handle = project.defs.create_int_grid_image().map(|image| {
                load_context.set_labeled_asset("int_grid_image", LoadedAsset::new(image))
            });

            let ldtk_asset = LdtkAsset {
                project,
                tileset_map,
                level_map,
                int_grid_image_handle,
            };
            load_context.set_default_asset(
                LoadedAsset::new(ldtk_asset)
                    .with_dependencies(tileset_rel_paths)
                    .with_dependencies(external_level_paths)
                    .with_dependencies(background_images),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

/// Secondary asset for loading ldtk files, specific to level data.
///
/// Loaded as a labeled asset when loading a standalone ldtk file with [LdtkAsset].
/// The label is just the level's identifier.
///
/// Loaded as a dependency to the [LdtkAsset] when loading an ldtk file with external levels.
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
