use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_ecs_tilemap::prelude::*;
use ldtk_rust::Project;

pub struct LevelIdentifier {
    pub identifier: String,
}

#[derive(TypeUuid)]
#[uuid = "ecfb87b7-9cd9-4970-8482-f2f68b770d31"]
pub struct LdtkAsset {
    pub project: Project,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<LdtkAsset>()
            .init_asset_loader::<LdtkLoader>();
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
                project.load_external_levels(format!(
                    "assets/{}",
                    load_context
                        .path()
                        .to_str()
                        .expect("ldtk path should be valid unicode")
                ));
            }
            let ldtk_asset = LdtkAsset { project };

            let parent_path = load_context.path().parent();

            load_context.set_default_asset(LoadedAsset::new(ldtk_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
