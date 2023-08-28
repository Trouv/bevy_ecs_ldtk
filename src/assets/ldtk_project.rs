use crate::{
    assets::{ldtk_path_to_asset_path, LdtkLevel},
    ldtk::{raw_level_accessor::RawLevelAccessor, LdtkJson, Level},
    resources::LevelSelection,
};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture,
};
use derive_getters::Getters;
use std::collections::HashMap;

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
    level_map: HashMap<String, Handle<LdtkLevel>>,
    /// Image used for rendering int grid colors.
    int_grid_image_handle: Option<Handle<Image>>,
}

impl RawLevelAccessor for LdtkProject {
    fn worlds(&self) -> &[crate::ldtk::World] {
        self.data.worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.data.root_levels()
    }
}

impl LdtkProject {
    /// Find a particular level using a [`LevelSelection`].
    ///
    /// Note: the returned level is the one existent in the [`LdtkProject`].
    /// This level will have "incomplete" data if you use LDtk's external levels feature.
    /// To always get full level data, you'll need to access `Assets<LdtkLevel>`.
    pub fn get_level(&self, level_selection: &LevelSelection) -> Option<&Level> {
        self.iter_raw_levels_with_indices()
            .find(|(i, l)| level_selection.is_match(i, l))
            .map(|(_, l)| l)
    }
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

            let mut external_level_paths = Vec::new();
            let mut level_map = HashMap::new();
            let mut background_images = Vec::new();
            if data.external_levels {
                for level in data.iter_raw_levels() {
                    if let Some(external_rel_path) = &level.external_rel_path {
                        let asset_path =
                            ldtk_path_to_asset_path(load_context.path(), external_rel_path);

                        external_level_paths.push(asset_path.clone());
                        level_map.insert(level.iid.clone(), load_context.get_handle(asset_path));
                    }
                }
            } else {
                for level in data.iter_raw_levels() {
                    let label = level.identifier.as_ref();

                    let mut background_image = None;
                    if let Some(rel_path) = &level.bg_rel_path {
                        let asset_path = ldtk_path_to_asset_path(load_context.path(), rel_path);
                        background_images.push(asset_path.clone());
                        background_image = Some(load_context.get_handle(asset_path));
                    }

                    let ldtk_level = LdtkLevel::new(level.clone(), background_image);
                    let level_handle =
                        load_context.set_labeled_asset(label, LoadedAsset::new(ldtk_level));

                    level_map.insert(level.iid.clone(), level_handle);
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

#[cfg(test)]
mod tests {
    use crate::ldtk::{raw_level_accessor::tests::sample_levels, World};

    use super::*;

    #[test]
    fn raw_level_accessor_implementation_is_transparent() {
        let [level_a, level_b, level_c, level_d] = sample_levels();

        let world_a = World {
            levels: vec![level_c.clone()],
            ..Default::default()
        };

        let world_b = World {
            levels: vec![level_d.clone()],
            ..Default::default()
        };

        let data = LdtkJson {
            worlds: vec![world_a, world_b],
            levels: vec![level_a.clone(), level_b.clone()],
            ..Default::default()
        };

        let project = LdtkProject {
            data: data.clone(),
            tileset_map: HashMap::default(),
            level_map: HashMap::default(),
            int_grid_image_handle: None,
        };

        assert_eq!(project.root_levels(), data.root_levels());
        assert_eq!(project.worlds(), data.worlds());
    }
}
