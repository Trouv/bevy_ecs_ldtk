use std::path::Path;

use crate::{
    assets::{
        ExternalLevelMetadata, LdtkProjectGetters, LdtkProjectWithMetadata, LevelIndices,
        LevelMetadata, LevelSelectionAccessor,
    },
    ldtk::{raw_level_accessor::RawLevelAccessor, LdtkJson, Level, World},
};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use derive_more::{From, TryInto};
use std::collections::HashMap;
use thiserror::Error;

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}

#[derive(Clone, Debug, PartialEq, From, TryInto, TypeUuid, TypePath)]
#[uuid = "00989906-69af-496f-a8a9-fdfef5c594f5"]
#[try_into(owned, ref)]
pub enum LdtkProject {
    Standalone(LdtkProjectWithMetadata<LevelMetadata>),
    Parent(LdtkProjectWithMetadata<ExternalLevelMetadata>),
}

impl LdtkProject {
    pub fn standalone(&self) -> &LdtkProjectWithMetadata<LevelMetadata> {
        self.try_into().unwrap()
    }

    pub fn parent(&self) -> &LdtkProjectWithMetadata<ExternalLevelMetadata> {
        self.try_into().unwrap()
    }
}

impl LdtkProjectGetters for LdtkProject {
    fn data(&self) -> &crate::ldtk::LdtkJson {
        match self {
            LdtkProject::Standalone(project) => project.data(),
            LdtkProject::Parent(project) => project.data(),
        }
    }

    fn tileset_map(&self) -> &std::collections::HashMap<i32, Handle<Image>> {
        match self {
            LdtkProject::Standalone(project) => project.tileset_map(),
            LdtkProject::Parent(project) => project.tileset_map(),
        }
    }

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>> {
        match self {
            LdtkProject::Standalone(project) => project.int_grid_image_handle(),
            LdtkProject::Parent(project) => project.int_grid_image_handle(),
        }
    }
}

impl RawLevelAccessor for LdtkProject {
    fn worlds(&self) -> &[World] {
        self.data().worlds()
    }

    fn root_levels(&self) -> &[crate::ldtk::Level] {
        self.data().root_levels()
    }
}

impl LevelSelectionAccessor for LdtkProject {
    fn get_indices_for_iid(&self, iid: &String) -> Option<&crate::prelude::LevelIndices> {
        match self {
            LdtkProject::Standalone(project) => project
                .level_map()
                .get(iid)
                .map(|level_metadata| level_metadata.indices()),
            LdtkProject::Parent(project) => project
                .level_map()
                .get(iid)
                .map(|external_level_metadata| external_level_metadata.metadata().indices()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkProjectLoaderError {
    #[error("LDtk project uses internal levels, but some level's layer_instances is null")]
    InternalLevelWithNullLayers,
    #[error("LDtk project uses external levels, but some level's external_rel_path is null")]
    ExternalLevelWithNullPath,
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

struct LoadExternalLevelMetadataResult<'a> {
    bg_image_path: Option<AssetPath<'a>>,
    external_level_path: AssetPath<'a>,
    metadata: ExternalLevelMetadata,
}

fn load_external_level_metadata<'a>(
    load_context: &LoadContext,
    level_indices: LevelIndices,
    level: &Level,
) -> Result<LoadExternalLevelMetadataResult<'a>, LdtkProjectLoaderError> {
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
            .ok_or(LdtkProjectLoaderError::ExternalLevelWithNullPath)?,
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
) -> Result<(), LdtkProjectLoaderError> {
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
                let mut level_map = HashMap::new();

                for (level_indices, level) in data.iter_raw_levels_with_indices() {
                    load_level_metadata_into_buffers(
                        load_context,
                        level_indices,
                        level,
                        &mut level_map,
                        &mut dependent_asset_paths,
                    )?;
                }

                LdtkProject::Standalone(LdtkProjectWithMetadata::new(
                    data,
                    tileset_map,
                    int_grid_image_handle,
                    level_map,
                ))
            } else {
                let mut level_map = HashMap::new();

                for (level_indices, level) in data.iter_raw_levels_with_indices() {
                    load_external_level_metadata_into_buffers(
                        load_context,
                        level_indices,
                        level,
                        &mut level_map,
                        &mut dependent_asset_paths,
                    )?;
                }

                LdtkProject::Parent(LdtkProjectWithMetadata::new(
                    data,
                    tileset_map,
                    int_grid_image_handle,
                    level_map,
                ))
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
