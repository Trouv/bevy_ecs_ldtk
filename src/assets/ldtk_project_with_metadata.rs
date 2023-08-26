use crate::{
    assets::{LevelIndices, LevelMetadata, LevelSelectionAccessor},
    ldtk::{
        loaded_level::LoadedLevel, raw_level_accessor::RawLevelAccessor, LdtkJson, Level, World,
    },
    resources::LevelSelection,
};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

use super::{ExternalLevelMetadata, LdtkExternalLevel, LdtkProjectGetters};

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

fn expect_level_loaded(level: &Level) -> LoadedLevel {
    LoadedLevel::try_from(level)
        .expect("LdtkProject construction should guarantee that internal levels are loaded")
}

/// Main asset for loading ldtk files.
///
/// Load your ldtk project with the asset server, then insert the handle into the
/// [`LdtkWorldBundle`].
///
/// [`LdtkWorldBundle`]: crate::components::LdtkWorldBundle
#[derive(Clone, Debug, PartialEq, TypeUuid, TypePath)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkProjectWithMetadata<L> {
    /// Raw ldtk project data.
    data: LdtkJson,
    /// Map from tileset uids to image handles for the loaded tileset.
    tileset_map: HashMap<i32, Handle<Image>>,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
    /// Map from level iids to level metadata.
    level_map: HashMap<String, L>,
}

impl<L> LdtkProjectGetters for LdtkProjectWithMetadata<L> {
    fn data(&self) -> &LdtkJson {
        &self.data
    }

    fn tileset_map(&self) -> &HashMap<i32, Handle<Image>> {
        &self.tileset_map
    }

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>> {
        &self.int_grid_image_handle
    }
}

impl<L> LdtkProjectWithMetadata<L> {
    pub(crate) fn new(
        data: LdtkJson,
        tileset_map: HashMap<i32, Handle<Image>>,
        int_grid_image_handle: Option<Handle<Image>>,
        level_map: HashMap<String, L>,
    ) -> Self {
        Self {
            data,
            tileset_map,
            int_grid_image_handle,
            level_map,
        }
    }

    pub fn level_map(&self) -> &HashMap<String, L> {
        &self.level_map
    }
}

impl<L> RawLevelAccessor for LdtkProjectWithMetadata<L> {
    fn root_levels(&self) -> &[Level] {
        self.data.root_levels()
    }

    fn worlds(&self) -> &[World] {
        self.data.worlds()
    }
}

impl LevelSelectionAccessor for LdtkProjectWithMetadata<LevelMetadata> {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&LevelIndices> {
        Some(self.level_map.get(iid)?.indices())
    }
}

impl LdtkProjectWithMetadata<LevelMetadata> {
    pub fn iter_loaded_levels(&self) -> impl Iterator<Item = LoadedLevel> {
        self.iter_raw_levels().map(expect_level_loaded)
    }

    pub fn get_loaded_level_by_indices(&self, indices: &LevelIndices) -> Option<LoadedLevel> {
        self.get_raw_level_at_indices(indices)
            .map(expect_level_loaded)
    }

    pub fn get_loaded_level_by_iid(&self, iid: &String) -> Option<LoadedLevel> {
        self.get_raw_level_by_iid(iid).map(expect_level_loaded)
    }

    pub fn find_loaded_level_by_level_selection(
        &self,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel> {
        self.find_raw_level_by_level_selection(level_selection)
            .map(expect_level_loaded)
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkProjectLoaderError {
    #[error("LDtk project uses external levels, use LdtkProjectParent asset instead")]
    ExternalLevelProject,
    #[error("LDtk project uses internal levels, but some level's layer_instances is null")]
    InternalLevelWithNullLayers,
}

#[derive(Default)]
pub struct LdtkProjectLoader;

struct LoadLevelMetadataResult<'a> {
    bg_image_path: Option<AssetPath<'a>>,
    external_level_path: Option<AssetPath<'a>>,
    level_metadata: LevelMetadata,
}

fn load_level_metadata<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
) -> Result<LoadLevelMetadataResult<'a>, LdtkProjectLoaderError> {
    let (bg_image_path, bg_image) = level
        .bg_rel_path
        .as_ref()
        .map(|rel_path| {
            let asset_path = ldtk_path_to_asset_path(load_context.path(), &rel_path);

            (
                Some(asset_path.clone()),
                Some(load_context.get_handle(asset_path)),
            )
        })
        .unwrap_or((None, None));

    if level.layer_instances.is_none() {
        Err(LdtkProjectLoaderError::InternalLevelWithNullLayers)?;
    }

    let level_metadata = LevelMetadata::new(bg_image, level_indices);

    Ok(LoadLevelMetadataResult {
        bg_image_path,
        level_metadata,
        external_level_path: None,
    })
}

fn load_level_metadata_into_buffers<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
    level_map: &mut HashMap<String, LevelMetadata>,
    dependent_asset_paths: &mut Vec<AssetPath<'a>>,
) -> Result<(), LdtkProjectLoaderError> {
    let LoadLevelMetadataResult {
        bg_image_path,
        external_level_path,
        level_metadata,
    } = load_level_metadata(load_context, level_indices, level)?;

    if let Some(bg_image_path) = bg_image_path {
        dependent_asset_paths.push(bg_image_path);
    }
    if let Some(external_level_path) = external_level_path {
        dependent_asset_paths.push(external_level_path);
    }
    level_map.insert(level.iid.clone(), level_metadata);

    Ok(())
}

impl AssetLoader for LdtkProjectLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: LdtkJson = serde_json::from_slice(bytes)?;

            if data.external_levels {
                Err(LdtkProjectLoaderError::ExternalLevelProject)?;
            }
            let mut level_map = HashMap::new();

            let mut dependent_asset_paths = Vec::new();

            for (level_index, level) in data.levels.iter().enumerate() {
                load_level_metadata_into_buffers(
                    load_context,
                    LevelIndices::in_root(level_index),
                    level,
                    &mut level_map,
                    &mut dependent_asset_paths,
                )?;
            }

            for (world_index, world) in data.worlds.iter().enumerate() {
                for (level_index, level) in world.levels.iter().enumerate() {
                    load_level_metadata_into_buffers(
                        load_context,
                        LevelIndices::in_world(world_index, level_index),
                        level,
                        &mut level_map,
                        &mut dependent_asset_paths,
                    )?;
                }
            }

            let mut tileset_map = HashMap::new();
            for tileset in &data.defs.tilesets {
                if let Some(tileset_path) = &tileset.rel_path {
                    let asset_path = ldtk_path_to_asset_path(load_context.path(), tileset_path);

                    dependent_asset_paths.push(asset_path.clone());
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

            let ldtk_asset = LdtkProjectWithMetadata {
                data,
                tileset_map,
                level_map,
                int_grid_image_handle,
            };

            load_context.set_default_asset(
                LoadedAsset::new(ldtk_asset).with_dependencies(dependent_asset_paths),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

impl LevelSelectionAccessor for LdtkProjectWithMetadata<ExternalLevelMetadata> {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&LevelIndices> {
        Some(self.level_map().get(iid)?.metadata().indices())
    }
}

impl LdtkProjectWithMetadata<ExternalLevelMetadata> {
    pub fn iter_external_levels<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
    ) -> impl Iterator<Item = LoadedLevel<'a>> {
        self.level_map()
            .values()
            .filter_map(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    pub fn get_external_level_by_indices<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        indices: &LevelIndices,
    ) -> Option<LoadedLevel<'a>> {
        self.get_external_level_by_iid(
            external_level_assets,
            &self.get_raw_level_at_indices(indices)?.iid,
        )
    }

    pub fn get_external_level_by_iid<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        iid: &String,
    ) -> Option<LoadedLevel<'a>> {
        self.level_map()
            .get(iid)
            .and_then(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    pub fn find_external_level_by_level_selection<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel<'a>> {
        match level_selection {
            LevelSelection::Iid(iid) => {
                self.get_external_level_by_iid(external_level_assets, iid.get())
            }
            LevelSelection::Indices(indices) => {
                self.get_external_level_by_indices(external_level_assets, indices)
            }
            _ => self.get_external_level_by_iid(
                external_level_assets,
                &self.find_raw_level_by_level_selection(level_selection)?.iid,
            ),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkParentProjectLoaderError {
    #[error("LDtk project uses internal levels, use LdtkProject asset instead")]
    InternalLevelProject,
    #[error("LDtk project uses external levels, but some level's external_rel_path is null")]
    ExternalLevelWithNullPath,
}

#[derive(Default)]
pub struct LdtkParentProjectLoader;

struct LoadExternalLevelMetadataResult<'a> {
    bg_image_path: Option<AssetPath<'a>>,
    external_level_path: AssetPath<'a>,
    metadata: ExternalLevelMetadata,
}

fn load_external_level_metadata<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
) -> Result<LoadExternalLevelMetadataResult<'a>, LdtkParentProjectLoaderError> {
    let (bg_image_path, bg_image) = level
        .bg_rel_path
        .as_ref()
        .map(|rel_path| {
            let asset_path = ldtk_path_to_asset_path(load_context.path(), &rel_path);

            (
                Some(asset_path.clone()),
                Some(load_context.get_handle(asset_path)),
            )
        })
        .unwrap_or((None, None));

    let external_level_path = ldtk_path_to_asset_path(
        load_context.path(),
        level
            .external_rel_path
            .as_ref()
            .ok_or(LdtkParentProjectLoaderError::ExternalLevelWithNullPath)?,
    );

    let external_handle = load_context.get_handle(external_level_path.clone());

    let metadata =
        ExternalLevelMetadata::new(LevelMetadata::new(bg_image, level_indices), external_handle);

    Ok(LoadExternalLevelMetadataResult {
        bg_image_path,
        metadata,
        external_level_path,
    })
}

fn load_external_level_metadata_into_buffers<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
    level_map: &mut HashMap<String, ExternalLevelMetadata>,
    dependent_asset_paths: &mut Vec<AssetPath<'a>>,
) -> Result<(), LdtkParentProjectLoaderError> {
    let LoadExternalLevelMetadataResult {
        bg_image_path,
        external_level_path,
        metadata,
    } = load_external_level_metadata(load_context, level_indices, level)?;

    if let Some(bg_image_path) = bg_image_path {
        dependent_asset_paths.push(bg_image_path);
    }

    dependent_asset_paths.push(external_level_path);
    level_map.insert(level.iid.clone(), metadata);

    Ok(())
}

impl AssetLoader for LdtkParentProjectLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: LdtkJson = serde_json::from_slice(bytes)?;

            if !data.external_levels {
                Err(LdtkParentProjectLoaderError::InternalLevelProject)?;
            }

            let mut level_map = HashMap::new();

            let mut dependent_asset_paths = Vec::new();

            for (level_index, level) in data.levels.iter().enumerate() {
                load_external_level_metadata_into_buffers(
                    load_context,
                    LevelIndices::in_root(level_index),
                    level,
                    &mut level_map,
                    &mut dependent_asset_paths,
                )?;
            }

            for (world_index, world) in data.worlds.iter().enumerate() {
                for (level_index, level) in world.levels.iter().enumerate() {
                    load_external_level_metadata_into_buffers(
                        load_context,
                        LevelIndices::in_world(world_index, level_index),
                        level,
                        &mut level_map,
                        &mut dependent_asset_paths,
                    )?;
                }
            }

            let mut tileset_map = HashMap::new();
            for tileset in &data.defs.tilesets {
                if let Some(tileset_path) = &tileset.rel_path {
                    let asset_path = ldtk_path_to_asset_path(load_context.path(), tileset_path);

                    dependent_asset_paths.push(asset_path.clone());
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

            let ldtk_asset = LdtkProjectWithMetadata {
                data,
                tileset_map,
                level_map,
                int_grid_image_handle,
            };

            load_context.set_default_asset(
                LoadedAsset::new(ldtk_asset).with_dependencies(dependent_asset_paths),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
