use crate::ldtk::{LdtkJson, Level};
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use std::{collections::HashMap, path::Path};

fn ldtk_path_to_asset_path<'a, 'b>(
    load_context: &LoadContext<'a>,
    rel_path: &String,
) -> AssetPath<'b> {
    load_context
        .path()
        .parent()
        .unwrap()
        .join(Path::new(rel_path))
        .into()
}

pub type TilesetMap = HashMap<i64, Handle<Texture>>;

#[derive(TypeUuid)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkAsset {
    pub project: LdtkJson,
    pub tileset_map: TilesetMap,
    pub external_levels: Vec<Handle<LdtkExternalLevel>>,
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
            let mut external_level_handles = Vec::new();
            if project.external_levels {
                for level in &project.levels {
                    if let Some(external_rel_path) = &level.external_rel_path {
                        let asset_path = ldtk_path_to_asset_path(load_context, &external_rel_path);

                        external_level_paths.push(asset_path.clone());
                        external_level_handles.push(load_context.get_handle(asset_path));
                    }
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
                external_levels: external_level_handles,
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
pub struct LdtkExternalLevel {
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
            let ldtk_level = LdtkExternalLevel {
                level: serde_json::from_slice(&bytes)?,
            };
            load_context.set_default_asset(LoadedAsset::new(ldtk_level));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}
