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
            half_width: 8.,
            half_height: 8.,
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
            if rect_a.right > rect_b.left
                && rect_a.left < rect_b.right
                && rect_a.top > rect_b.bottom
                && rect_a.bottom < rect_b.top
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
