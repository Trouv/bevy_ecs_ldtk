use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use avian2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: LinearVelocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderDensity,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::rectangle(12., 28.),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: Collider::rectangle(10., 10.),
                rigid_body: RigidBody::Kinematic,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: Collider::rectangle(16., 16.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderDensity(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub collision_events: CollisionEventsEnabled,
    pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: Collider::rectangle(16., 16.),
                sensor: Sensor,
                rotation_constraints,
                collision_events: CollisionEventsEnabled,
            }
        } else {
            SensorBundle::default()
        }
    }
}
