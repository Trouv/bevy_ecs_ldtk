use crate::{
    assets::{LdtkJsonWithMetadata, LevelMetadata, LevelMetadataAccessor},
    ldtk::{LdtkJson, Level},
    prelude::RawLevelAccessor,
};
use bevy::reflect::Reflect;
use derive_more::{From, TryInto};

#[cfg(feature = "external_levels")]
use crate::assets::ExternalLevelMetadata;

/// LDtk json data and level metadata for both internal- and external-level projects.
///
/// We need to abstract over these cases to allow them in the same asset type: [`LdtkProject`].
/// All methods that are available in both cases are available here.
/// However, methods exclusive to each case require accessing the internal type.
/// These include methods for obtaining [`LoadedLevel`]s.
/// See the [`LoadedLevel`]-accessing methods in the following impls:
/// - [standalone projects](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<LevelMetadata>)
/// - [parent projects](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<ExternalLevelMetadata>)
///
/// Note that this type's variants are under different feature flags.
/// At least one of these feature flags needs to be enabled for the plugin to compile.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
/// [`LoadedLevel`]: crate::ldtk::loaded_level::LoadedLevel
#[derive(Clone, Debug, PartialEq, From, TryInto, Reflect)]
#[try_into(owned, ref)]
pub enum LdtkProjectData {
    /// LDtk data for a standalone project (uses internal levels).
    ///
    /// This is only available under the `internal_levels` feature.
    #[cfg(feature = "internal_levels")]
    Standalone(LdtkJsonWithMetadata<LevelMetadata>),
    /// LDtk data for a parent project (uses external levels).
    ///
    /// This is only available under the `external_levels` feature.
    #[cfg(feature = "external_levels")]
    Parent(LdtkJsonWithMetadata<ExternalLevelMetadata>),
}

impl LdtkProjectData {
    /// Raw ldtk json data.
    pub fn json_data(&self) -> &LdtkJson {
        match self {
            #[cfg(feature = "internal_levels")]
            LdtkProjectData::Standalone(project) => project.json_data(),
            #[cfg(feature = "external_levels")]
            LdtkProjectData::Parent(project) => project.json_data(),
        }
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<LevelMetadata>`].
    /// For use on internal-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if this is not [`LdtkProjectData::Standalone`].
    /// This shouldn't occur if the project uses internal levels.
    ///
    /// [`LdtkJsonWithMetadata<LevelMetadata>`]: LdtkJsonWithMetadata
    /// [`LoadedLevel`]: crate::assets::loaded_level::LoadedLevel
    #[cfg(feature = "internal_levels")]
    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<LevelMetadata> {
        self.try_into().unwrap()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<ExternalLevelMetadata>`].
    /// For use on external-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if this is not [`LdtkProjectData::Parent`].
    /// This shouldn't occur if the project uses external levels.
    ///
    /// [`LdtkJsonWithMetadata<ExternalLevelMetadata>`]: LdtkJsonWithMetadata
    /// [`LoadedLevel`]: crate::assets::loaded_level::LoadedLevel
    #[cfg(feature = "external_levels")]
    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevelMetadata> {
        self.try_into().unwrap()
    }
}

impl RawLevelAccessor for LdtkProjectData {
    fn worlds(&self) -> &[crate::ldtk::World] {
        self.json_data().worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.json_data().root_levels()
    }
}

impl LevelMetadataAccessor for LdtkProjectData {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        match self {
            #[cfg(feature = "internal_levels")]
            LdtkProjectData::Standalone(project) => project.get_level_metadata_by_iid(iid),
            #[cfg(feature = "external_levels")]
            LdtkProjectData::Parent(project) => project.get_level_metadata_by_iid(iid),
        }
    }
}
