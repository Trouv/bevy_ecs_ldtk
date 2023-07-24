//! Assets for loading ldtk files.

use bevy::asset::AssetPath;
use std::path::Path;

mod ldtk_asset_plugin;
pub use ldtk_asset_plugin::LdtkAssetPlugin;

mod ldtk_level;
pub use ldtk_level::LdtkLevel;

mod ldtk_project;
pub use ldtk_project::LdtkProject;

fn ldtk_path_to_asset_path<'b>(ldtk_path: &Path, rel_path: &str) -> AssetPath<'b> {
    ldtk_path.parent().unwrap().join(Path::new(rel_path)).into()
}
