use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_ecs_tilemap::prelude::*;
use futures::{
    executor::block_on,
    stream::{self, StreamExt},
};
use ldtk_rust::{Level, Project};
use std::{collections::HashMap, path::Path};

mod assets;
mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Component)]
pub enum LevelSelection {
    Identifier(String),
    Index(usize),
    Uid(i64),
}

impl Default for LevelSelection {
    fn default() -> Self {
        LevelSelection::Index(0)
    }
}

#[derive(TypeUuid)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkAsset {
    pub project: Project,
    pub tileset_map: HashMap<i64, Handle<Texture>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct LdtkPlugin;

#[derive(Clone, Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_handle: Handle<LdtkAsset>,
    pub level_selection: LevelSelection,
    pub map: Map,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<LdtkAsset>()
            .init_asset_loader::<LdtkLoader>()
            .add_asset::<assets::LdtkLevel>()
            .init_asset_loader::<assets::LdtkLevelLoader>()
            .add_system(systems::process_loaded_ldtk);
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
            let mut project: Project = serde_json::from_slice(bytes)?;

            if project.external_levels {
                block_on(load_external_levels_with_context(
                    load_context,
                    &mut project,
                ));
            }

            let mut tileset_rel_paths = Vec::new();
            let mut tileset_map = HashMap::new();
            for tileset in &project.defs.tilesets {
                let asset_path: AssetPath = load_context
                    .path()
                    .parent()
                    .unwrap()
                    .join(Path::new(&tileset.rel_path))
                    .into();
                tileset_rel_paths.push(asset_path.clone());

                tileset_map.insert(tileset.uid, load_context.get_handle(asset_path));
            }

            let ldtk_asset = LdtkAsset {
                project,
                tileset_map,
            };
            load_context.set_default_asset(
                LoadedAsset::new(ldtk_asset).with_dependencies(tileset_rel_paths),
            );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

async fn load_external_levels_with_context<'a>(
    load_context: &mut LoadContext<'a>,
    project: &mut Project,
) {
    if project.external_levels {
        let level_rel_paths: Vec<&Path> = project
            .levels
            .iter()
            .map(|l| Path::new(l.external_rel_path.as_ref().expect("missing level")))
            .collect();

        project.levels = stream::iter(level_rel_paths)
            .map(|p| load_external_level_with_context(load_context, p))
            .buffered(10)
            .map(|l| l.expect("Error reading level"))
            .collect()
            .await;
    }
}

async fn load_external_level_with_context<'a>(
    load_context: &LoadContext<'a>,
    level_rel_path: &Path,
) -> anyhow::Result<Level> {
    let asset_path = load_context.path().parent().unwrap().join(level_rel_path);
    let level_bytes = load_context.read_asset_bytes(asset_path).await?;

    Ok(serde_json::from_slice(&level_bytes)?)
}
