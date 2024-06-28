use std::{io, path::Path};

use crate::{
    assets::{
        LdtkJsonWithMetadata, LdtkProjectData, LevelIndices, LevelMetadata, LevelMetadataAccessor,
    },
    ldtk::{raw_level_accessor::RawLevelAccessor, LdtkJson, Level},
};
use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::Reflect,
    utils::ConditionalSendFuture,
};
use derive_getters::Getters;
use derive_more::From;
use path_clean::PathClean;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(feature = "internal_levels")]
use crate::assets::InternalLevels;

#[cfg(feature = "external_levels")]
use crate::assets::{ExternalLevelMetadata, ExternalLevels};

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path
        .parent()
        .unwrap()
        .join(Path::new(rel_path))
        .clean()
        .into()
}

/// Main asset for loading LDtk project data.
///
/// # Accessing level data
/// This type provides many methods for accessing level data.
/// The correct method for you will vary depending on whether or not you need "complete" level
/// data, and if so, whether or not your project uses internal levels or external levels.
///
/// ## Raw vs loaded levels
/// There are a couple main flavors that level data can have - raw and loaded.
///
/// Raw levels don't have any type guarantee that the level data is complete or incomplete.
/// Level data may be incomplete and contain no layer instances if external levels are enabled.
/// However, even in this case, a raw level is sufficient if you don't need any layer data.
/// Raw levels are represented by the [`Level`] type from LDtk.
/// See [`RawLevelAccessor`] and [`LevelMetadataAccessor`] for some methods that access raw levels.
///
/// On the other hand, loaded levels are type-guaranteed to have complete level data.
/// Loaded levels are represented by the [`LoadedLevel`] type.
/// Methods for accessing loaded levels vary depending on if the levels are internal or external.
///
/// ## Accessing internal and external loaded levels
/// By default, LDtk stores level data inside the main project file.
/// You have the option to store level data externally, where each level gets its own file.
/// In this case, some of the level data remains available in the project file, but not layer data.
/// See the [previous section](LdtkProject#raw-vs-loaded-levels) for more details.
///
/// Level data stored so differently on disk results in a similar difference when loaded in memory.
/// In the external case, an entirely different asset type [`LdtkExternalLevel`] comes into play.
/// So, methods for accessing loaded levels vary between the two cases.
///
/// If you know that your project uses internal levels, you can coerce it as a "standalone project".
/// To do this, use [`LdtkProject::as_standalone`].
/// With that, you can use these [`loaded_level` accessors].
///
/// If you know that your project uses external levels, you can coerce it as a "parent project".
/// To do this, use [`LdtkProject::as_parent`].
/// You will also need the [`LdtkExternalLevel`] asset collection.
/// With these, you can use these [`external_level` accessors].
///
/// [`LoadedLevel`]: crate::ldtk::loaded_level::LoadedLevel
/// [`LdtkExternalLevel`]: crate::assets::LdtkExternalLevel
/// [`loaded_level` accessors]: LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<InternalLevels>
/// [`external_level` accessors]: LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<ExternalLevels>
#[derive(Clone, Debug, PartialEq, From, Getters, Reflect, Asset)]
pub struct LdtkProject {
    /// LDtk json data and level metadata.
    data: LdtkProjectData,
    /// Map from tileset uids to image handles for the loaded tileset.
    tileset_map: HashMap<i32, Handle<Image>>,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
}

impl LdtkProject {
    /// Construct a new [`LdtkProject`].
    ///
    /// Private to preserve type guarantees about loaded levels.
    fn new(
        data: LdtkProjectData,
        tileset_map: HashMap<i32, Handle<Image>>,
        int_grid_image_handle: Option<Handle<Image>>,
    ) -> LdtkProject {
        LdtkProject {
            data,
            tileset_map,
            int_grid_image_handle,
        }
    }

    /// Raw ldtk json data.
    pub fn json_data(&self) -> &LdtkJson {
        self.data.json_data()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<InternalLevels>`].
    /// For use on internal-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if `self.data()` is not [`LdtkProjectData::Standalone`].
    /// This shouldn't occur if the project uses internal levels.
    ///
    /// [`LdtkJsonWithMetadata<InternalLevels>`]: LdtkJsonWithMetadata
    #[cfg(feature = "internal_levels")]
    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<InternalLevels> {
        self.data.as_standalone()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<ExternalLevels>`].
    /// For use on external-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if `self.data()` is not [`LdtkProjectData::Parent`].
    /// This shouldn't occur if the project uses external levels.
    ///
    /// [`LdtkJsonWithMetadata<ExternalLevels>`]: LdtkJsonWithMetadata
    #[cfg(feature = "external_levels")]
    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevels> {
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

/// Errors that can occur when loading an [`LdtkProject`] asset.
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum LdtkProjectLoaderError {
    /// Encountered IO error reading LDtk project
    #[error("encountered IO error reading LDtk project: {0}")]
    Io(#[from] io::Error),
    /// Unable to deserialize LDtk project
    #[error("unable to deserialize LDtk project: {0}")]
    Deserialize(#[from] serde_json::Error),
    /// LDtk project uses internal levels, but the `internal_levels` feature is disabled.
    #[error("LDtk project uses internal levels, but the internal_levels feature is disabled")]
    InternalLevelsDisabled,
    /// LDtk project uses external levels, but the `external_levels` feature is disabled.
    #[error("LDtk project uses external levels, but the external_levels feature is disabled")]
    ExternalLevelsDisabled,
    /// LDtk project uses internal levels, but some level's `layer_instances` is null.
    #[error("LDtk project uses internal levels, but some level's layer_instances is null")]
    InternalLevelWithNullLayers,
    /// LDtk project uses external levels, but some level's `external_rel_path` is null.
    #[error("LDtk project uses external levels, but some level's external_rel_path is null")]
    ExternalLevelWithNullPath,
}

/// AssetLoader for [`LdtkProject`].
#[derive(Default)]
pub struct LdtkProjectLoader;

fn load_level_metadata(
    load_context: &mut LoadContext,
    level_indices: LevelIndices,
    level: &Level,
    expect_level_loaded: bool,
) -> Result<LevelMetadata, LdtkProjectLoaderError> {
    let bg_image = level.bg_rel_path.as_ref().map(|rel_path| {
        let asset_path = ldtk_path_to_asset_path(load_context.path(), rel_path);

        load_context.load(asset_path)
    });

    if expect_level_loaded && level.layer_instances.is_none() {
        Err(LdtkProjectLoaderError::InternalLevelWithNullLayers)?;
    }

    let level_metadata = LevelMetadata::new(bg_image, level_indices);

    Ok(level_metadata)
}

#[cfg(feature = "external_levels")]
fn load_external_level_metadata(
    load_context: &mut LoadContext,
    level_indices: LevelIndices,
    level: &Level,
) -> Result<ExternalLevelMetadata, LdtkProjectLoaderError> {
    let level_metadata = load_level_metadata(load_context, level_indices, level, false)?;

    let external_level_path = ldtk_path_to_asset_path(
        load_context.path(),
        level
            .external_rel_path
            .as_ref()
            .ok_or(LdtkProjectLoaderError::ExternalLevelWithNullPath)?,
    );

    let external_handle = load_context.load(external_level_path.clone());

    Ok(ExternalLevelMetadata::new(level_metadata, external_handle))
}

impl AssetLoader for LdtkProjectLoader {
    type Asset = LdtkProject;
    type Settings = ();
    type Error = LdtkProjectLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<
        Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>,
    > {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let data: LdtkJson = serde_json::from_slice(&bytes)?;

            let mut tileset_map: HashMap<i32, Handle<Image>> = HashMap::new();
            for tileset in &data.defs.tilesets {
                if let Some(tileset_path) = &tileset.rel_path {
                    let asset_path = ldtk_path_to_asset_path(load_context.path(), tileset_path);

                    tileset_map.insert(tileset.uid, load_context.load(asset_path));
                } else if tileset.embed_atlas.is_some() {
                    warn!("Ignoring LDtk's Internal_Icons. They cannot be displayed due to their license.");
                } else {
                    let identifier = &tileset.identifier;
                    warn!("{identifier} tileset cannot be loaded, it has a null relative path.");
                }
            }

            let int_grid_image_handle = data
                .defs
                .create_int_grid_image()
                .map(|image| load_context.add_labeled_asset("int_grid_image".to_string(), image));

            let ldtk_project = if data.external_levels {
                #[cfg(feature = "external_levels")]
                {
                    let mut level_map = HashMap::new();

                    for (level_indices, level) in data.iter_raw_levels_with_indices() {
                        let level_metadata =
                            load_external_level_metadata(load_context, level_indices, level)?;

                        level_map.insert(level.iid.clone(), level_metadata);
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
                        let level_metadata =
                            load_level_metadata(load_context, level_indices, level, true)?;

                        level_map.insert(level.iid.clone(), level_metadata);
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

            Ok(ldtk_project)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use derive_more::Constructor;
    use fake::{Dummy, Fake, Faker};
    use rand::Rng;

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Constructor)]
    pub struct LdtkProjectFaker<F>
    where
        LdtkProjectData: Dummy<F>,
    {
        ldtk_project_data_faker: F,
    }

    impl<F> Dummy<LdtkProjectFaker<F>> for LdtkProject
    where
        LdtkProjectData: Dummy<F>,
    {
        fn dummy_with_rng<R: Rng + ?Sized>(config: &LdtkProjectFaker<F>, rng: &mut R) -> Self {
            let data: LdtkProjectData = config.ldtk_project_data_faker.fake_with_rng(rng);
            let tileset_map = data
                .json_data()
                .defs
                .tilesets
                .iter()
                .map(|tileset| (tileset.uid, Handle::weak_from_u128(Faker.fake())))
                .collect();

            LdtkProject {
                data,
                tileset_map,
                int_grid_image_handle: Some(Handle::weak_from_u128(Faker.fake())),
            }
        }
    }

    #[test]
    fn normalizes_asset_paths() {
        let resolve_path = |project_path, rel_path| {
            let asset_path = ldtk_path_to_asset_path(Path::new(project_path), rel_path);
            asset_path.path().to_owned()
        };

        assert_eq!(
            resolve_path("project.ldtk", "images/tiles.png"),
            Path::new("images/tiles.png")
        );
        assert_eq!(
            resolve_path("projects/sub/project.ldtk", "../images/tiles.png"),
            Path::new("projects/images/tiles.png")
        );
        assert_eq!(
            resolve_path("projects/sub/project.ldtk", "../../tiles.png"),
            Path::new("tiles.png")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn normalizes_windows_asset_paths() {
        let resolve_path = |project_path, rel_path| {
            let asset_path = ldtk_path_to_asset_path(Path::new(project_path), rel_path);
            asset_path.path().to_owned()
        };

        assert_eq!(
            resolve_path("projects\\sub/project.ldtk", "../images/tiles.png"),
            Path::new("projects/images/tiles.png")
        );
        assert_eq!(
            resolve_path("projects\\sub/project.ldtk", "../../images/tiles.png"),
            Path::new("images/tiles.png")
        );
        assert_eq!(
            resolve_path("projects/sub\\project.ldtk", "../../tiles.png"),
            Path::new("tiles.png")
        );
    }

    #[cfg(feature = "internal_levels")]
    mod internal_levels {
        use crate::{
            assets::{
                ldtk_json_with_metadata::tests::LdtkJsonWithMetadataFaker,
                ldtk_project_data::internal_level_tests::StandaloneLdtkProjectDataFaker,
            },
            ldtk::fake::{LoadedLevelsFaker, MixedLevelsLdtkJsonFaker},
        };

        use super::*;

        impl Dummy<InternalLevels> for LdtkProject {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &InternalLevels, rng: &mut R) -> Self {
                LdtkProjectFaker {
                    ldtk_project_data_faker: InternalLevels,
                }
                .fake_with_rng(rng)
            }
        }

        #[test]
        fn json_data_accessor_is_transparent() {
            let project: LdtkProject = InternalLevels.fake();

            assert_eq!(project.json_data(), project.data().json_data());
        }

        #[test]
        fn raw_level_accessor_implementation_is_transparent() {
            let project: LdtkProject = LdtkProjectFaker::new(StandaloneLdtkProjectDataFaker::new(
                LdtkJsonWithMetadataFaker::new(MixedLevelsLdtkJsonFaker::new(
                    LoadedLevelsFaker::default(),
                    4..8,
                )),
            ))
            .fake();

            assert_eq!(project.root_levels(), project.json_data().root_levels());
            assert_eq!(project.worlds(), project.json_data().worlds());
        }

        #[test]
        fn level_metadata_accessor_implementation_is_transparent() {
            let project: LdtkProject = InternalLevels.fake();

            for level in &project.json_data().levels {
                assert_eq!(
                    project.get_level_metadata_by_iid(&level.iid),
                    project.data().get_level_metadata_by_iid(&level.iid),
                );
            }

            assert_eq!(
                project.get_level_metadata_by_iid(&"This_level_doesnt_exist".to_string()),
                None
            );
        }
    }

    #[cfg(feature = "external_levels")]
    mod external_levels {
        use crate::{
            assets::{
                ldtk_json_with_metadata::tests::LdtkJsonWithMetadataFaker,
                ldtk_project_data::external_level_tests::ParentLdtkProjectDataFaker,
            },
            ldtk::fake::{LoadedLevelsFaker, MixedLevelsLdtkJsonFaker},
        };

        use super::*;

        impl Dummy<ExternalLevels> for LdtkProject {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &ExternalLevels, rng: &mut R) -> Self {
                LdtkProjectFaker {
                    ldtk_project_data_faker: ExternalLevels,
                }
                .fake_with_rng(rng)
            }
        }

        #[test]
        fn json_data_accessor_is_transparent() {
            let project: LdtkProject = ExternalLevels.fake();

            assert_eq!(project.json_data(), project.data().json_data());
        }

        #[test]
        fn raw_level_accessor_implementation_is_transparent() {
            let project: LdtkProject = LdtkProjectFaker::new(ParentLdtkProjectDataFaker::new(
                LdtkJsonWithMetadataFaker::new(MixedLevelsLdtkJsonFaker::new(
                    LoadedLevelsFaker::default(),
                    4..8,
                )),
            ))
            .fake();

            assert_eq!(project.root_levels(), project.json_data().root_levels());
            assert_eq!(project.worlds(), project.json_data().worlds());
        }

        #[test]
        fn level_metadata_accessor_implementation_is_transparent() {
            let project: LdtkProject = ExternalLevels.fake();

            for level in &project.json_data().levels {
                assert_eq!(
                    project.get_level_metadata_by_iid(&level.iid),
                    project.data().get_level_metadata_by_iid(&level.iid),
                );
            }

            assert_eq!(
                project.get_level_metadata_by_iid(&"This_level_doesnt_exist".to_string()),
                None
            );
        }
    }
}
