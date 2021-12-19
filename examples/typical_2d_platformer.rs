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
        .insert_resource(physics::MaxVelocity { value: 1000. })
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(detect_climb_range)
        .add_system(ignore_gravity_if_climbing)
        .add_system(patrol)
        .add_system(patrol_setup)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 1)
        .register_ldtk_int_cell_for_layer::<LadderBundle>("Collisions", 2)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 3)
        .register_ldtk_entity_for_layer::<PlayerBundle>("Entities", "Player")
        .register_ldtk_entity_for_layer::<MobBundle>("Entities", "Mob")
        .run();
}

mod physics {
    use bevy::prelude::*;
    use std::collections::HashMap;

    pub struct BasicPhysicsPlugin;

    impl Plugin for BasicPhysicsPlugin {
        fn build(&self, app: &mut App) {
            app.add_event::<CollisionEvent>()
                .add_system_to_stage(CoreStage::PreUpdate, apply_velocity.label("apply_velocity"))
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    detect_collision.after("apply_velocity"),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    gravity.label("gravity").after("apply_velocity"),
                )
                .add_system_to_stage(
                    CoreStage::PreUpdate,
                    uncollide_rigid_bodies.after("gravity"),
                );
        }
    }

    #[derive(Copy, Clone, PartialEq, Debug, Component)]
    pub struct RectangleCollider {
        pub half_width: f32,
        pub half_height: f32,
    }

    impl Default for RectangleCollider {
        fn default() -> Self {
            RectangleCollider {
                half_width: 7.5,
                half_height: 7.5,
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
    pub enum RigidBody {
        Dynamic,
        Static,
        Sensor,
    }

    impl Default for RigidBody {
        fn default() -> Self {
            RigidBody::Static
        }
    }

    #[derive(Copy, Clone, PartialEq, Debug, Default)]
    pub struct Gravity {
        pub value: f32,
    }

    #[derive(Copy, Clone, PartialEq, Debug, Default)]
    pub struct MaxVelocity {
        pub value: f32,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct CollisionEvent {
        pub entity: Entity,
        pub other_entity: Entity,
        pub overlap: Vec2,
    }

    #[derive(Copy, Clone, Debug, Default, Component)]
    pub struct Velocity {
        pub value: Vec3,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
    pub struct IgnoreGravity;

    pub fn detect_collision(
        query: Query<(Entity, &Transform, &RectangleCollider)>,
        mut writer: EventWriter<CollisionEvent>,
    ) {
        let mut collider_rectangles = Vec::new();
        query.for_each(|(entity, transform, rectangle_collider)| {
            let half_width = transform.scale.x * rectangle_collider.half_width;
            let half_height = transform.scale.y * rectangle_collider.half_height;
            collider_rectangles.push((
                entity,
                Rect {
                    left: transform.translation.x - half_width,
                    bottom: transform.translation.y - half_height,
                    right: transform.translation.x + half_width,
                    top: transform.translation.y + half_height,
                },
            ));
        });

        for (i, (entity_a, rect_a)) in collider_rectangles.iter().enumerate() {
            for (entity_b, rect_b) in collider_rectangles[i + 1..].iter() {
                if rect_a.right >= rect_b.left
                    && rect_a.left <= rect_b.right
                    && rect_a.top >= rect_b.bottom
                    && rect_a.bottom <= rect_b.top
                {
                    let overlap_x = if f32::abs(rect_a.right - rect_b.left)
                        <= f32::abs(rect_a.left - rect_b.right)
                    {
                        rect_a.right - rect_b.left
                    } else {
                        rect_a.left - rect_b.right
                    };

                    let overlap_y = if f32::abs(rect_a.top - rect_b.bottom)
                        <= f32::abs(rect_a.bottom - rect_b.top)
                    {
                        rect_a.top - rect_b.bottom
                    } else {
                        rect_a.bottom - rect_b.top
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

    pub fn uncollide_rigid_bodies(
        mut query: Query<(&mut Transform, &mut Velocity, &RigidBody)>,
        mut collisions: EventReader<CollisionEvent>,
    ) {
        let mut adjustments: HashMap<Entity, Vec2> = HashMap::new();

        for collision in collisions.iter() {
            if let Ok((_, _, other_rigid_body)) = query.get_mut(collision.other_entity) {
                if *other_rigid_body != RigidBody::Sensor {
                    if let Ok((_, _, RigidBody::Dynamic)) = query.get_mut(collision.entity) {
                        let current_adjustment = adjustments
                            .entry(collision.entity)
                            .or_insert_with(|| Vec2::default());

                        if f32::abs(current_adjustment.x) < f32::abs(collision.overlap.x)
                            && f32::abs(current_adjustment.y) < f32::abs(collision.overlap.y)
                        {
                            if f32::abs(collision.overlap.x) <= f32::abs(collision.overlap.y) {
                                current_adjustment.x -= collision.overlap.x;
                            } else {
                                current_adjustment.y -= collision.overlap.y;
                            }
                        }
                    }
                }
            }
        }

        for (entity, adjustment) in adjustments.iter() {
            if let Ok((mut transform, mut velocity, _)) = query.get_mut(*entity) {
                transform.translation += Vec3::from((*adjustment, 0.));
                if adjustment.x != 0. {
                    velocity.value.x = 0.;
                }
                if adjustment.y != 0. {
                    velocity.value.y = 0.;
                }
            }
        }
    }

    pub fn gravity(
        mut query: Query<(&mut Velocity, &RigidBody), Without<IgnoreGravity>>,
        gravity: Res<Gravity>,
        time: Res<Time>,
    ) {
        query.for_each_mut(|(mut velocity, rigid_body)| {
            if *rigid_body == RigidBody::Dynamic {
                velocity.value.y += gravity.value * time.delta_seconds();
            }
        });
    }

    pub fn apply_velocity(
        mut query: Query<(&mut Velocity, &mut Transform, &RigidBody)>,
        time: Res<Time>,
        max_velocity: Res<MaxVelocity>,
    ) {
        query.for_each_mut(|(mut velocity, mut transform, rigid_body)| {
            if velocity.value.length() > max_velocity.value {
                velocity.value = velocity.value.normalize() * max_velocity.value;
            }

            if *rigid_body == RigidBody::Dynamic {
                transform.translation += velocity.value * time.delta_seconds();
            }
        });
    }
}

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
                    half_width: 8.,
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
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        match int_grid_cell.value {
            2 => ColliderBundle {
                collider: physics::RectangleCollider {
                    half_width: 0.5,
                    half_height: 7.5,
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
