use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(detect_collision)
        .run();
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct RectangleCollider {
    half_width: f32,
    half_height: f32,
}

#[derive(Copy, Clone, Debug, Component)]
enum RigidBody {
    Dynamic,
    Static,
    Sensor,
}

#[derive(Copy, Clone, Debug)]
struct CollisionEvent {
    entities: (Entity, Entity),
    normal: Vec2,
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct Velocity {
    value: Vec3,
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct Acceleration {
    value: Vec3,
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct Force {
    value: Vec3,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    let map_entity = commands.spawn().id();
    let transform = Transform::default();
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        transform,
        ..Default::default()
    });
}

fn detect_collision(
    query: Query<(Entity, &Transform, &Velocity, &RectangleCollider)>,
    mut writer: EventWriter<CollisionEvent>,
) {
    let mut collider_rectangles = Vec::new();
    query.for_each(|(entity, transform, velocity, rectangle_collider)| {
        let half_width = transform.scale.x * rectangle_collider.half_width;
        let half_height = transform.scale.y * rectangle_collider.half_height;
        collider_rectangles.push((
            entity,
            velocity,
            Vec4::new(
                transform.translation.x - half_width,
                transform.translation.y - half_height,
                half_width * 2.,
                half_height * 2.,
            ),
        ));
    });

    for (i, (entity_a, velocity_a, rect_a)) in collider_rectangles.iter().enumerate() {
        for (entity_b, velocity_b, rect_b) in collider_rectangles[i + 1..].iter() {
            if rect_a.x + rect_a.z >= rect_b.x
                && rect_a.x <= rect_b.x + rect_b.z
                && rect_a.y + rect_a.w >= rect_b.y
                && rect_a.y <= rect_b.y + rect_b.z
            {
                writer.send(CollisionEvent {
                    entities: (*entity_a, *entity_b),
                    normal: Vec2::new(0., 0.),
                });
            }
        }
    }
}
