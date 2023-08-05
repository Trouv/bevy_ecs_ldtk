use crate::{
    assets::level_map::LevelMap,
    ldtk::{loaded_level::LoadedLevel, LdtkJson, Level},
    resources::LevelSelection,
    LevelIid,
};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use derive_getters::Getters;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[cfg(not(feature = "external_levels"))]
use crate::assets::level_map::InternalLevel;

#[cfg(feature = "external_levels")]
use crate::assets::{level_map::ExternalLevel, LdtkExternalLevel};

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

/// Main asset for loading ldtk files.
///
/// Load your ldtk project with the asset server, then insert the handle into the
/// [`LdtkWorldBundle`].
///
/// [`LdtkWorldBundle`]: crate::components::LdtkWorldBundle
#[derive(Clone, Debug, PartialEq, TypeUuid, TypePath, Getters)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkProject {
    /// Raw ldtk project data.
    data: LdtkJson,
    /// Map from tileset uids to image handles for the loaded tileset.
    tileset_map: HashMap<i32, Handle<Image>>,
    /// Map from level iids to level handles.
    level_map: LevelMap,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
}

impl LdtkProject {
    /// Get an iterator of all the levels in the LDtk file.
    ///
    /// This abstraction avoids compatibility issues between pre-multi-world and post-multi-world
    /// LDtk projects.
    ///
    /// Note: the returned levels are the ones existent in the [`LdtkProject`].
    /// These levels will have "incomplete" data if you use LDtk's external levels feature.
    /// To always get full level data, you'll need to access `Assets<LdtkLevel>`.
    pub fn iter_raw_levels(&self) -> impl Iterator<Item = &Level> {
        self.data.iter_raw_levels()
    }

    /// Find a particular level using a [`LevelSelection`].
    ///
    /// Note: the returned level is the one existent in the [`LdtkProject`].
    /// This level will have "incomplete" data if you use LDtk's external levels feature.
    /// To always get full level data, you'll need to access `Assets<LdtkLevel>`.
    pub fn find_raw_level(&self, level_selection: &LevelSelection) -> Option<&Level> {
        self.iter_raw_levels()
            .enumerate()
            .find(|(i, l)| level_selection.is_match(i, l))
            .map(|(_, l)| l)
    }

    #[cfg(not(feature = "external_levels"))]
    pub fn iter_loaded_levels(&self) -> impl Iterator<Item = LoadedLevel> {
        self.iter_raw_levels().map(|level| {
            LoadedLevel::try_from(level).expect(
                "construction of LDtkProject should guarantee that internal levels are loaded.",
            )
        })
    }

    #[cfg(feature = "external_levels")]
    pub fn iter_loaded_levels<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
    ) -> impl Iterator<Item = LoadedLevel<'a>> {
        self.level_map.values().map(|external_level| {
            external_level_assets
                .get(external_level.level_handle())
                .map(|level_asset| level_asset.data())
                .expect("TODO")
        })
    }

    #[cfg(not(feature = "external_levels"))]
    pub fn get_loaded_level(&self, iid: &LevelIid) -> Option<LoadedLevel> {
        self.level_map
            .get(iid)
            .map(|internal_level| {
                self.iter_raw_levels()
                    .nth(*internal_level.level_index())
                    .expect("internal level index should be valid")
            })
            .map(|raw| {
                LoadedLevel::try_from(raw).expect(
                    "construction of LDtkProject should guarantee that internal levels are loaded",
                )
            })
    }

    #[cfg(feature = "external_levels")]
    pub fn get_loaded_level<'a>(
        &'a self,
        iid: &LevelIid,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
    ) -> Option<LoadedLevel<'a>> {
        self.level_map.get(iid).map(|external_level| {
            external_level_assets
                .get(external_level.level_handle())
                .map(|level_asset| level_asset.data())
                .expect("TODO")
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkProjectLoaderError {
    #[error("external_levels feature enabled, but LDtk project uses internal levels")]
    InternalLevelProject,
    #[error("LDtk project uses external levels, but external_levels feature not enabled")]
    ExternalLevelProject,
    #[error("LDtk project uses internal levels, but the level's layers is null")]
    InternalLevelWithNullLayers,
}

#[derive(Default)]
pub struct LdtkProjectLoader;

impl AssetLoader for LdtkProjectLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: LdtkJson = serde_json::from_slice(bytes)?;

            let mut background_images = Vec::new();

            if data.external_levels && !cfg!(feature = "external_levels") {
                Err(LdtkProjectLoaderError::ExternalLevelProject)?;
            } else if !data.external_levels && cfg!(feature = "external_levels") {
                Err(LdtkProjectLoaderError::InternalLevelProject)?;
            }

            let mut level_map = LevelMap::new();

            #[allow(unused_mut)]
            let mut external_level_paths = Vec::new();

            #[allow(unused_variables)]
            for (level_index, level) in data.iter_raw_levels().enumerate() {
                let mut bg_image = None;
                if let Some(rel_path) = &level.bg_rel_path {
                    let asset_path = ldtk_path_to_asset_path(load_context.path(), rel_path);

                    background_images.push(asset_path.clone());
                    bg_image = Some(load_context.get_handle(asset_path));
                }

                #[cfg(feature = "external_levels")]
                {
                    let asset_path = ldtk_path_to_asset_path(
                        load_context.path(),
                        &level.external_rel_path.as_ref().expect("TODO"),
                    );

                    external_level_paths.push(asset_path.clone());

                    let level_handle = load_context.get_handle(asset_path);
                    level_map.insert(
                        LevelIid::new(level.iid.clone()),
                        ExternalLevel::new(bg_image, level_handle),
                    );
                }

                #[cfg(not(feature = "external_levels"))]
                {
                    if level.layer_instances.is_none() {
                        Err(LdtkProjectLoaderError::InternalLevelWithNullLayers)?;
                    }

                    level_map.insert(
                        LevelIid::new(level.iid.clone()),
                        InternalLevel::new(bg_image, level_index),
                    );
                }
            }

            let mut tileset_rel_paths = Vec::new();
            let mut tileset_map = HashMap::new();
            for tileset in &data.defs.tilesets {
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

            let int_grid_image_handle = data.defs.create_int_grid_image().map(|image| {
                load_context.set_labeled_asset("int_grid_image", LoadedAsset::new(image))
            });

            let ldtk_asset = LdtkProject {
                data,
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
