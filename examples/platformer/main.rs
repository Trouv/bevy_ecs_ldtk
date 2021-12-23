use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use std::collections::HashSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(physics::BasicPhysicsPlugin)
        .insert_resource(physics::Gravity { value: -2000. })
        .insert_resource(physics::MaxVelocity { value: 700. })
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(detect_climb_range)
        .add_system(ignore_gravity_if_climbing)
        .add_system(patrol)
        .add_system(patrol_setup)
        .add_system(camera_fit_inside_current_level)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 1)
        .register_ldtk_int_cell_for_layer::<LadderBundle>("Collisions", 2)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 3)
        .register_ldtk_entity_for_layer::<PlayerBundle>("Entities", "Player")
        .register_ldtk_entity_for_layer::<MobBundle>("Entities", "Mob")
        .register_ldtk_entity_for_layer::<ChestBundle>("Entities", "Chest")
        .run();
}

mod physics;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    let map_entity = commands.spawn().id();
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        ..Default::default()
    });
}

#[derive(Copy, Clone, Debug, Default, Bundle, LdtkIntCell)]
struct ColliderBundle {
    collider: physics::RectangleCollider,
    rigid_body: physics::RigidBody,
    velocity: physics::Velocity,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: physics::RectangleCollider {
                    half_width: 4.,
                    half_height: 11.,
                },
                rigid_body: physics::RigidBody::Dynamic,
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: physics::RectangleCollider {
                    half_width: 5.,
                    half_height: 5.,
                },
                rigid_body: physics::RigidBody::Dynamic,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: physics::RectangleCollider {
                    half_width: 8.,
                    half_height: 8.,
                },
                rigid_body: physics::RigidBody::Dynamic,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        match int_grid_cell.value {
            2 => ColliderBundle {
                collider: physics::RectangleCollider {
                    half_width: 8.,
                    half_height: 8.,
                },
                rigid_body: physics::RigidBody::Sensor,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
enum Climber {
    OutOfClimbRange,
    InClimbRange,
    Climbing,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_bundle("player.png")]
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    #[bundle]
    collider_bundle: ColliderBundle,
    player: Player,
    climber: Climber,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Climbable;

impl Default for Climber {
    fn default() -> Climber {
        Climber::OutOfClimbRange
    }
}

#[derive(Copy, Clone, Debug, Default, Bundle, LdtkIntCell)]
struct LadderBundle {
    #[from_int_grid_cell]
    #[bundle]
    collider_bundle: ColliderBundle,
    climbable: Climbable,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Enemy;

#[derive(Clone, PartialEq, Debug, Default, Component)]
struct Patrol {
    points: Vec<Vec2>,
    index: usize,
    forward: bool,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct MobBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    collider_bundle: ColliderBundle,
    enemy: Enemy,
    ignore_gravity: physics::IgnoreGravity,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    patrol: Patrol,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct ChestBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    collider_bundle: ColliderBundle,
}

fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut physics::Velocity, &mut Climber), With<Player>>,
) {
    for (mut velocity, mut climber) in query.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.value.x = (right - left) * 200.;

        if *climber == Climber::InClimbRange
            && (input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::S))
        {
            *climber = Climber::Climbing;
        }

        if *climber == Climber::Climbing {
            let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

            velocity.value.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) {
            velocity.value.y = 450.;
            *climber = Climber::InClimbRange;
        }
    }
}

fn detect_climb_range(
    mut climbers: Query<(Entity, &mut Climber)>,
    climbables: Query<&Climbable>,
    mut collisions: EventReader<physics::CollisionEvent>,
) {
    let mut climbers_in_range = HashSet::new();
    for collision in collisions.iter() {
        if climbers.get_mut(collision.entity).is_ok()
            && climbables
                .get_component::<Climbable>(collision.other_entity)
                .is_ok()
        {
            climbers_in_range.insert(collision.entity);
        }
    }

    for (entity, mut climber) in climbers.iter_mut() {
        if !climbers_in_range.contains(&entity) {
            *climber = Climber::OutOfClimbRange;
        } else if *climber != Climber::Climbing {
            *climber = Climber::InClimbRange;
        }
    }
}

fn ignore_gravity_if_climbing(
    mut commands: Commands,
    query: Query<(Entity, &Climber), Changed<Climber>>,
) {
    for (entity, climber) in query.iter() {
        match *climber {
            Climber::Climbing => {
                commands.entity(entity).insert(physics::IgnoreGravity);
            }
            _ => {
                commands.entity(entity).remove::<physics::IgnoreGravity>();
            }
        }
    }
}

fn patrol_setup(
    mut query: Query<(&mut Patrol, &EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (mut patrol, entity_instance, transform) in query.iter_mut() {
        patrol.points.push(transform.translation.into());

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "patrol".to_string())
            .unwrap();
        if let Some(serde_json::Value::Array(ldtk_points)) = &ldtk_patrol.value {
            for ldtk_point in ldtk_points {
                if let serde_json::Value::Object(ldtk_point) = ldtk_point {
                    if let (
                        Some(serde_json::Value::Number(x)),
                        Some(serde_json::Value::Number(y)),
                    ) = (ldtk_point.get("cx"), ldtk_point.get("cy"))
                    {
                        if let (Some(x), Some(y)) = (x.as_i64(), y.as_i64()) {
                            let mut grid_offset = IVec2::new(x as i32, y as i32)
                                - IVec2::from_slice(entity_instance.grid.as_slice());

                            grid_offset.y = -grid_offset.y;

                            let pixel_offset = grid_offset.as_vec2() * Vec2::splat(16.);

                            patrol
                                .points
                                .push(Vec2::from(transform.translation) + pixel_offset);
                        }
                    }
                }
            }
        }

        patrol.forward = true;
        patrol.index = 1;
    }
}

fn patrol(mut query: Query<(&mut Transform, &mut physics::Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in query.iter_mut() {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity = Vec3::from((
            (patrol.points[patrol.index] - Vec2::from(transform.translation)).normalize() * 75.,
            0.,
        ));

        if new_velocity.dot(velocity.value) < 0. {
            if patrol.index == 0 {
                patrol.forward = true;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity = Vec3::from((
                (patrol.points[patrol.index] - Vec2::from(transform.translation)).normalize() * 75.,
                0.,
            ));
        }

        velocity.value = new_velocity;
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

fn camera_fit_inside_current_level(
    mut transform_query_set: QuerySet<(
        QueryState<(
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        )>,
        QueryState<&Transform, With<Player>>,
    )>,
    ldtk_map_query: Query<(&Handle<LdtkAsset>, &LevelSelection)>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
) {
    let (ldtk_handle, level_selection) = ldtk_map_query.single();

    if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
        if let Some((_, level)) = ldtk_asset
            .project
            .levels
            .iter()
            .enumerate()
            .find(|(i, l)| level_selection.is_match(i, l))
        {
            if let Ok(Transform {
                translation: player_translation,
                ..
            }) = transform_query_set.q1().get_single()
            {
                let player_translation = player_translation.clone();
                let mut camera_query = transform_query_set.q0();

                let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

                let level_ratio = level.px_wid as f32 / level.px_hei as f32;

                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = 0.;
                orthographic_projection.left = 0.;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    orthographic_projection.top = level.px_hei as f32;
                    orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                    camera_transform.translation.x = (player_translation.x
                        - orthographic_projection.right / 2.)
                        .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                } else {
                    // level is taller than the screen
                    orthographic_projection.right = level.px_wid as f32;
                    orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                    camera_transform.translation.y = (player_translation.y
                        - orthographic_projection.top / 2.)
                        .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                }
            }
        }
    }
}
