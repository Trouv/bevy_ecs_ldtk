use crate::{
    ldtk::{LdtkJson, Level},
    resources::LevelSelection,
};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use std::{collections::HashMap, path::Path};

fn ldtk_path_to_asset_path<'a, 'b>(
    load_context: &LoadContext<'a>,
    rel_path: &str,
) -> AssetPath<'b> {
    load_context
        .path()
        .parent()
        .unwrap()
        .join(Path::new(rel_path))
        .into()
}

pub type TilesetMap = HashMap<i32, Handle<Image>>;
pub type LevelMap = HashMap<i32, Handle<LdtkLevel>>;

#[derive(TypeUuid)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkAsset {
    pub project: LdtkJson,
    pub tileset_map: TilesetMap,
    pub level_map: LevelMap,
}

impl LdtkAsset {
    pub fn world_height(&self) -> i32 {
        let mut world_height = 0;
        for level in &self.project.levels {
            world_height = world_height.max(level.world_y + level.px_hei);
        }

        world_height
    }

    pub fn get_level(&self, level_selection: &LevelSelection) -> Option<&Level> {
        self.project
            .levels
            .iter()
            .enumerate()
            .find(|(i, l)| level_selection.is_match(i, l))
            .map(|(_, l)| l)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkLoader;

impl AssetLoader for LdtkLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let project: LdtkJson = serde_json::from_slice(bytes)?;

            let mut external_level_paths = Vec::new();
            let mut level_map = HashMap::new();
            if project.external_levels {
                for level in &project.levels {
                    if let Some(external_rel_path) = &level.external_rel_path {
                        let asset_path = ldtk_path_to_asset_path(load_context, external_rel_path);

                        external_level_paths.push(asset_path.clone());
                        level_map.insert(level.uid, load_context.get_handle(asset_path));
                    }
                }
            } else {
                for level in &project.levels {
                    let label = level.identifier.as_ref();
                    let ldtk_level = LdtkLevel {
                        level: level.clone(),
                    };
                    let level_handle =
                        load_context.set_labeled_asset(label, LoadedAsset::new(ldtk_level));

                    level_map.insert(level.uid, level_handle);
                }
            }

            let mut tileset_rel_paths = Vec::new();
            let mut tileset_map = HashMap::new();
            for tileset in &project.defs.tilesets {
                let asset_path = ldtk_path_to_asset_path(load_context, &tileset.rel_path);

                tileset_rel_paths.push(asset_path.clone());
                tileset_map.insert(tileset.uid, load_context.get_handle(asset_path));
            }

            let ldtk_asset = LdtkAsset {
                project,
                tileset_map,
                level_map,
            };
            load_context.set_default_asset(
                LoadedAsset::new(ldtk_asset)
                    .with_dependencies(tileset_rel_paths)
                    .with_dependencies(external_level_paths),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

#[derive(TypeUuid)]
#[uuid = "5448469b-2134-44f5-a86c-a7b829f70a0c"]
pub struct LdtkLevel {
    pub level: Level,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LdtkLevelLoader;

impl AssetLoader for LdtkLevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let ldtk_level = LdtkLevel {
                level: serde_json::from_slice(bytes)?,
            };
            load_context.set_default_asset(LoadedAsset::new(ldtk_level));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
