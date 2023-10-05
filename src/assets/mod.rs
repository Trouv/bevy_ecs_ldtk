//! Assets for loading ldtk files.

use bevy::asset::AssetPath;
use std::path::Path;

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

mod ldtk_level;
pub use ldtk_level::LdtkLevel;

mod ldtk_project;
pub use ldtk_project::LdtkProject;

mod level_indices;
pub use level_indices::LevelIndices;

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}
