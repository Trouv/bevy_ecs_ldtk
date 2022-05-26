//! Assets and AssetLoaders for loading ldtk files.

use crate::{ldtk, resources::LevelSelection};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
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

            let ldtk_asset = LdtkAsset {
                project,
                tileset_map,
                level_map,
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
#[derive(TypeUuid)]
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
