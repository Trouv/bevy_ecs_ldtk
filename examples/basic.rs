use bevy::prelude::*;
use bevy_ecs_ldtk::*;
use serde_json;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .init_resource::<State>()
        .add_startup_system(setup)
        .add_system(print_on_load)
        .run();
}

#[derive(Default)]
struct State {
    handle: Handle<LdtkAsset>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("levels.ldtk");
}

fn print_on_load(mut state: ResMut<State>, ldtk_assets: ResMut<Assets<LdtkAsset>>) {
    let ldtk_asset = ldtk_assets.get(&state.handle);
    if state.printed || ldtk_asset.is_none() {
        return;
    }

    println!(
        "ldtk asset loaded: {}",
        serde_json::to_string(&ldtk_asset.unwrap().project).unwrap()
    );
    state.printed = true;
}
