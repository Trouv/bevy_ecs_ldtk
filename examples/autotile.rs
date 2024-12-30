/// This example showcases the autotiling functionality.
///
/// The "autotile.ldtk" project contains a single int grid layer, on which some rudimentary autotiling has been defined.
/// This app allows the user to change a cell's value by pressing on it with the mouse.mut commands: Commands
///
/// If the autotiling is implemented correctly, the drawn tiles should stay correctly autotiled even as tiles as updated.
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Resource, Debug)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_cursor_pos, click_update))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(CursorPos::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(300.0, 240.0, 0.0),
        OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("autotile.ldtk").into(),
        ..default()
    });
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
// Taken from: https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Ok(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

fn click_update(
    cursor_pos: Res<CursorPos>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    mut cell_q: Query<&mut IntGridCell>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        // Only do things on left button presses.
        return;
    }

    // Finding the highlighted tile is taken from: https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/mouse_to_tile.rs
    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        // Grab the cursor position from the `Res<CursorPos>`
        let cursor_pos: Vec2 = cursor_pos.0;
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        // Once we have a world position we can transform it into a possible tile position.
        let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        else {
            // Cursor outside the tilemap bounds.
            continue;
        };

        // Now fetch the tile entity that corresponds to the tile position we got.
        let Some(tile_entity) = tile_storage.get(&tile_pos) else {
            panic!("got tile pos that are missing from tile storage: {tile_pos:?}");
        };

        // And finally fetch the int grid cell from the abovementioned entity.
        let Ok(mut int_grid_cell) = cell_q.get_mut(tile_entity) else {
            panic!("got tile pos that do not correspond to an int_grid_cell: {tile_pos:?}");
        };

        // Invert the value on the cell.
        int_grid_cell.value = match int_grid_cell.value {
            1 => 2,
            2 => 1,
            x => panic!("unexpected int grid value: {x}"),
        };
    }
}
