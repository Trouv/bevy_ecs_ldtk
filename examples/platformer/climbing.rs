use avian2d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::colliders::SensorBundle;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub climbable: Climbable,
}

pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<Entity, With<Climbable>>,
    mut collision_starts: MessageReader<CollisionStart>,
    mut collision_ends: MessageReader<CollisionEnd>,
) {
    for CollisionStart {
        collider1,
        collider2,
        ..
    } in collision_starts.read()
    {
        if let (Ok(mut climber), Ok(climbable)) =
            (climbers.get_mut(*collider1), climbables.get(*collider2))
        {
            climber.intersecting_climbables.insert(climbable);
        }
        if let (Ok(mut climber), Ok(climbable)) =
            (climbers.get_mut(*collider2), climbables.get(*collider1))
        {
            climber.intersecting_climbables.insert(climbable);
        };
    }

    for CollisionEnd {
        collider1,
        collider2,
        ..
    } in collision_ends.read()
    {
        if let (Ok(mut climber), Ok(climbable)) =
            (climbers.get_mut(*collider1), climbables.get(*collider2))
        {
            climber.intersecting_climbables.remove(&climbable);
        }

        if let (Ok(mut climber), Ok(climbable)) =
            (climbers.get_mut(*collider2), climbables.get(*collider1))
        {
            climber.intersecting_climbables.remove(&climbable);
        }
    }
}

pub fn ignore_gravity_if_climbing(
    mut query: Query<(&Climber, &mut GravityScale), Changed<Climber>>,
) {
    for (climber, mut gravity_scale) in &mut query {
        if climber.climbing {
            gravity_scale.0 = 0.0;
        } else {
            gravity_scale.0 = 1.0;
        }
    }
}

pub struct ClimbingPlugin;

impl Plugin for ClimbingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_climb_range)
            .add_systems(Update, ignore_gravity_if_climbing)
            .register_ldtk_int_cell::<LadderBundle>(2);
    }
}
