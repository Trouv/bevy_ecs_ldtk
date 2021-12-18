use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(detect_collision)
        .add_system(uncollide_rigid_bodies)
        .add_system(gravity)
        .add_system(apply_velocity)
        .run();
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct RectangleCollider {
    half_width: f32,
    half_height: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
enum RigidBody {
    Dynamic,
    Static,
    Sensor,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Gravity {
    value: f32,
}

#[derive(Copy, Clone, Debug)]
struct CollisionEvent {
    entity: Entity,
    other_entity: Entity,
    overlap: Vec2,
}

#[derive(Copy, Clone, Debug, Default, Component)]
struct Velocity {
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
            let top_a = rect_a.y + rect_a.w;
            let right_a = rect_a.x + rect_a.z;
            let bottom_a = rect_a.y;
            let left_a = rect_a.x;

            let top_b = rect_b.y + rect_b.w;
            let right_b = rect_b.x + rect_b.z;
            let bottom_b = rect_b.y;
            let left_b = rect_b.x;

            if right_a >= left_b && left_a <= right_b && top_a >= bottom_b && bottom_a <= top_b {
                let overlap_x = if f32::abs(right_a - left_b) <= f32::abs(left_a - right_b) {
                    right_a - left_b
                } else {
                    left_a - right_b
                };

                let overlap_y = if f32::abs(top_a - bottom_b) <= f32::abs(bottom_a - top_b) {
                    top_a - bottom_b
                } else {
                    bottom_a - top_b
                };

                writer.send(CollisionEvent {
                    entity: *entity_a,
                    other_entity: *entity_b,
                    overlap: Vec2::new(overlap_x, overlap_y),
                });

                writer.send(CollisionEvent {
                    entity: *entity_b,
                    other_entity: *entity_a,
                    overlap: Vec2::new(-overlap_x, -overlap_y),
                });
            }
        }
    }
}

fn uncollide_rigid_bodies(
    mut query: Query<(&mut Transform, &mut Velocity, &RigidBody)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        if let Ok((_, _, other_rigid_body)) = query.get_mut(collision.other_entity) {
            if *other_rigid_body != RigidBody::Sensor {
                if let Ok((mut transform, mut velocity, RigidBody::Dynamic)) =
                    query.get_mut(collision.entity)
                {
                    if f32::abs(collision.overlap.x) <= f32::abs(collision.overlap.y) {
                        transform.translation.x -= collision.overlap.x;
                        velocity.value.x = 0.;
                    } else {
                        transform.translation.y -= collision.overlap.y;
                        velocity.value.y = 0.;
                    }
                }
            }
        }
    }
}

fn gravity(mut query: Query<(&mut Velocity, &RigidBody)>, gravity: Res<Gravity>, time: Res<Time>) {
    query.for_each_mut(|(mut velocity, rigid_body)| {
        if *rigid_body == RigidBody::Dynamic {
            velocity.value.y += gravity.value * time.delta_seconds();
        }
    });
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform, &RigidBody)>, time: Res<Time>) {
    query.for_each_mut(|(velocity, mut transform, rigid_body)| {
        if *rigid_body == RigidBody::Dynamic {
            transform.translation += velocity.value * time.delta_seconds();
        }
    });
}
