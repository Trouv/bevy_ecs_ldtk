use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(physics::Gravity { value: -5. })
        .add_event::<physics::CollisionEvent>()
        .add_system(physics::detect_collision)
        .add_system(physics::uncollide_rigid_bodies)
        .add_system(physics::gravity)
        .add_system(physics::apply_velocity)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 1)
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collisions", 3)
        .register_ldtk_entity_for_layer::<PlayerBundle>("Entities", "Player")
        .run();
}

mod physics {
    use bevy::prelude::*;
    use std::collections::HashMap;

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

    #[derive(Copy, Clone, Debug)]
    pub struct CollisionEvent {
        entity: Entity,
        other_entity: Entity,
        overlap: Vec2,
    }

    #[derive(Copy, Clone, Debug, Default, Component)]
    pub struct Velocity {
        value: Vec3,
    }

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

                    debug!("{:?} and {:?} collided", rect_a, rect_b);

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
            debug!("collision event: {:?}", collision);
            if let Ok((_, _, other_rigid_body)) = query.get_mut(collision.other_entity) {
                debug!("found other entity");
                if *other_rigid_body != RigidBody::Sensor {
                    if let Ok((_, _, RigidBody::Dynamic)) = query.get_mut(collision.entity) {
                        debug!("found entity");
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

        debug!("adjustments: {:?}", adjustments);

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
        mut query: Query<(&mut Velocity, &RigidBody)>,
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
        mut query: Query<(&Velocity, &mut Transform, &RigidBody)>,
        time: Res<Time>,
    ) {
        query.for_each_mut(|(velocity, mut transform, rigid_body)| {
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
    let transform = Transform::default();
    commands.entity(map_entity).insert_bundle(LdtkMapBundle {
        ldtk_handle,
        map: Map::new(0u16, map_entity),
        transform,
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
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_bundle("player.png")]
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    #[bundle]
    collider_bundle: ColliderBundle,
}
