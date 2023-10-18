//! Assets and related items for loading LDtk files.

mod ldtk_asset_plugin;
pub use ldtk_asset_plugin::LdtkAssetPlugin;

mod level_metadata;
pub use level_metadata::LevelMetadata;

#[cfg(feature = "external_levels")]
pub use level_metadata::ExternalLevelMetadata;

mod level_locale;

#[cfg(feature = "internal_levels")]
pub use level_locale::InternalLevels;

#[cfg(feature = "external_levels")]
pub use level_locale::ExternalLevels;

mod level_metadata_accessor;
pub use level_metadata_accessor::LevelMetadataAccessor;

#[cfg(feature = "external_levels")]
mod ldtk_external_level;

#[cfg(feature = "external_levels")]
pub use ldtk_external_level::LdtkExternalLevel;

mod ldtk_json_with_metadata;
pub use ldtk_json_with_metadata::LdtkJsonWithMetadata;

mod ldtk_project_data;
pub use ldtk_project_data::LdtkProjectData;

mod ldtk_project;
pub use ldtk_project::LdtkProject;

mod level_indices;
pub use level_indices::LevelIndices;
