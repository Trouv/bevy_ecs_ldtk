use std::path::Path;

use crate::{
    assets::{
        LdtkJsonWithMetadata, LdtkProjectData, LevelIndices, LevelMetadata, LevelMetadataAccessor,
    },
    ldtk::{raw_level_accessor::RawLevelAccessor, LdtkJson, Level},
};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use derive_getters::Getters;
use derive_more::{Constructor, From};
use std::collections::HashMap;
use thiserror::Error;

#[cfg(feature = "external_levels")]
use crate::assets::ExternalLevelMetadata;

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

/// Main asset for loading LDtk project data.
#[derive(Clone, Debug, PartialEq, From, TypeUuid, TypePath, Getters, Constructor)]
#[uuid = "43571891-8570-4416-903f-582efe3426ac"]
pub struct LdtkProject {
    /// LDtk json data and level metadata.
    data: LdtkProjectData,
    /// Map from tileset uids to image handles for the loaded tileset.
    tileset_map: HashMap<i32, Handle<Image>>,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
}

impl LdtkProject {
    /// Raw ldtk json data.
    pub fn json_data(&self) -> &LdtkJson {
        self.data.json_data()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<LevelMetadata>`].
    /// For use on internal-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if `self.data()` is not [`LdtkProjectData::Standalone`].
    /// This shouldn't occur if the project uses internal levels.
    ///
    /// [`LdtkJsonWithMetadata<LevelMetadata>`]: LdtkJsonWithMetadata
    /// [`LoadedLevel`]: crate::assets::loaded_level::LoadedLevel
    #[cfg(feature = "internal_levels")]
    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<LevelMetadata> {
        self.data.as_standalone()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<ExternalLevelMetadata>`].
    /// For use on external-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if `self.data()` is not [`LdtkProjectData::Parent`].
    /// This shouldn't occur if the project uses external levels.
    ///
    /// [`LdtkJsonWithMetadata<ExternalLevelMetadata>`]: LdtkJsonWithMetadata
    /// [`LoadedLevel`]: crate::assets::loaded_level::LoadedLevel
    #[cfg(feature = "external_levels")]
    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevelMetadata> {
        self.data.as_parent()
    }
}

impl RawLevelAccessor for LdtkProject {
    fn worlds(&self) -> &[crate::ldtk::World] {
        self.data.worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.data.root_levels()
    }
}

impl LevelMetadataAccessor for LdtkProject {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        self.data.get_level_metadata_by_iid(iid)
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkProjectLoaderError {
    #[error("LDtk project uses internal levels, but the internal_levels feature is disabled")]
    InternalLevelsDisabled,
    #[error("LDtk project uses external levels, but the external_levels feature is disabled")]
    ExternalLevelsDisabled,
    #[error("LDtk project uses internal levels, but some level's layer_instances is null")]
    InternalLevelWithNullLayers,
    #[error("LDtk project uses external levels, but some level's external_rel_path is null")]
    ExternalLevelWithNullPath,
}

#[derive(Default)]
pub struct LdtkProjectLoader;

struct LoadLevelMetadataResult<'a, L> {
    dependent_asset_paths: Vec<AssetPath<'a>>,
    level_metadata: L,
}

fn load_level_metadata<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
    expect_level_loaded: bool,
) -> Result<LoadLevelMetadataResult<'a, LevelMetadata>, LdtkProjectLoaderError> {
    let (bg_image_path, bg_image) = level
        .bg_rel_path
        .as_ref()
        .map(|rel_path| {
            let asset_path = ldtk_path_to_asset_path(load_context.path(), rel_path);

            (
                Some(asset_path.clone()),
                Some(load_context.get_handle(asset_path)),
            )
        })
        .unwrap_or((None, None));

    if expect_level_loaded && level.layer_instances.is_none() {
        Err(LdtkProjectLoaderError::InternalLevelWithNullLayers)?;
    }

    let level_metadata = LevelMetadata::new(bg_image, level_indices);

    Ok(LoadLevelMetadataResult {
        dependent_asset_paths: bg_image_path.into_iter().collect(),
        level_metadata,
    })
}

#[cfg(feature = "external_levels")]
fn load_external_level_metadata<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
) -> Result<LoadLevelMetadataResult<'a, ExternalLevelMetadata>, LdtkProjectLoaderError> {
    let LoadLevelMetadataResult {
        level_metadata,
        mut dependent_asset_paths,
    } = load_level_metadata(load_context, level_indices, level, false)?;

    let external_level_path = ldtk_path_to_asset_path(
        load_context.path(),
        level
            .external_rel_path
            .as_ref()
            .ok_or(LdtkProjectLoaderError::ExternalLevelWithNullPath)?,
    );

    let external_handle = load_context.get_handle(external_level_path.clone());
    dependent_asset_paths.push(external_level_path);

    Ok(LoadLevelMetadataResult {
        level_metadata: ExternalLevelMetadata::new(level_metadata, external_handle),
        dependent_asset_paths,
    })
}

impl AssetLoader for LdtkProjectLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let data: LdtkJson = serde_json::from_slice(bytes)?;

            let mut dependent_asset_paths = Vec::new();

            let mut tileset_map: HashMap<i32, Handle<Image>> = HashMap::new();
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

            let ldtk_project = if data.external_levels {
                #[cfg(feature = "external_levels")]
                {
                    let mut level_map = HashMap::new();

                    for (level_indices, level) in data.iter_raw_levels_with_indices() {
                        let LoadLevelMetadataResult {
                            level_metadata,
                            dependent_asset_paths: new_asset_paths,
                        } = load_external_level_metadata(load_context, level_indices, level)?;

                        level_map.insert(level.iid.clone(), level_metadata);
                        dependent_asset_paths.extend(new_asset_paths);
                    }

                    LdtkProject::new(
                        LdtkProjectData::Parent(LdtkJsonWithMetadata::new(data, level_map)),
                        tileset_map,
                        int_grid_image_handle,
                    )
                }

                #[cfg(not(feature = "external_levels"))]
                {
                    Err(LdtkProjectLoaderError::ExternalLevelsDisabled)?
                }
            } else {
                #[cfg(feature = "internal_levels")]
                {
                    let mut level_map = HashMap::new();

                    for (level_indices, level) in data.iter_raw_levels_with_indices() {
                        let LoadLevelMetadataResult {
                            level_metadata,
                            dependent_asset_paths: new_asset_paths,
                        } = load_level_metadata(load_context, level_indices, level, true)?;

                        level_map.insert(level.iid.clone(), level_metadata);
                        dependent_asset_paths.extend(new_asset_paths);
                    }

                    LdtkProject::new(
                        LdtkProjectData::Standalone(LdtkJsonWithMetadata::new(data, level_map)),
                        tileset_map,
                        int_grid_image_handle,
                    )
                }

                #[cfg(not(feature = "internal_levels"))]
                {
                    Err(LdtkProjectLoaderError::InternalLevelsDisabled)?
                }
            };

            load_context.set_default_asset(
                LoadedAsset::new(ldtk_project).with_dependencies(dependent_asset_paths),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
