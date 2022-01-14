use crate::components::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use heron::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

pub fn pause_physics_during_load(
    mut level_events: EventReader<LevelEvent>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber), With<Player>>,
) {
    for (mut velocity, mut climber) in query.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linear.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::S) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

            velocity.linear.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) {
            velocity.linear.y = 450.;
            climber.climbing = false;
        }
    }
}

pub fn detect_climb_range(
    mut climbers: Query<&mut Climber>,
    climbables: Query<&Climbable>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b) => {
                if let Ok(mut climber) = climbers.get_mut(collider_a.rigid_body_entity()) {
                    if climbables.get(collider_b.rigid_body_entity()).is_ok() {
                        climber
                            .intersecting_climbables
                            .insert(collider_b.rigid_body_entity());
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(collider_b.rigid_body_entity()) {
                    if climbables.get(collider_a.rigid_body_entity()).is_ok() {
                        climber
                            .intersecting_climbables
                            .insert(collider_a.rigid_body_entity());
                    }
                }
            }
            CollisionEvent::Stopped(collider_a, collider_b) => {
                if let Ok(mut climber) = climbers.get_mut(collider_a.rigid_body_entity()) {
                    if climbables.get(collider_b.rigid_body_entity()).is_ok() {
                        climber
                            .intersecting_climbables
                            .remove(&collider_b.rigid_body_entity());
                    }
                }

                if let Ok(mut climber) = climbers.get_mut(collider_b.rigid_body_entity()) {
                    if climbables.get(collider_a.rigid_body_entity()).is_ok() {
                        climber
                            .intersecting_climbables
                            .remove(&collider_a.rigid_body_entity());
                    }
                }
            }
        }
    }
}

pub fn ignore_gravity_if_climbing(
    mut commands: Commands,
    query: Query<(Entity, &Climber), Changed<Climber>>,
) {
    for (entity, climber) in query.iter() {
        if climber.climbing {
            commands
                .entity(entity)
                .insert(RigidBody::KinematicVelocityBased);
        } else {
            commands.entity(entity).insert(RigidBody::Dynamic);
        }
    }
}

pub fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in query.iter_mut() {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity = Vec3::from((
            (patrol.points[patrol.index] - Vec2::from(transform.translation.truncate()))
                .normalize()
                * 75.,
            0.,
        ));

        if new_velocity.dot(velocity.linear) < 0. {
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
                (patrol.points[patrol.index] - Vec2::from(transform.translation.truncate()))
                    .normalize()
                    * 75.,
                0.,
            ));
        }

        velocity.linear = new_velocity;
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = player_translation.clone();

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, &level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                bottom: level_transform.translation.y,
                top: level_transform.translation.y + ldtk_level.level.px_hei as f32,
                left: level_transform.translation.x,
                right: level_transform.translation.x + ldtk_level.level.px_wid as f32,
            };

            for player_transform in player_query.iter() {
                if player_transform.translation.x < level_bounds.right
                    && player_transform.translation.x > level_bounds.left
                    && player_transform.translation.y < level_bounds.top
                    && player_transform.translation.y > level_bounds.bottom
                {
                    if !level_selection.is_match(&0, &ldtk_level.level) {
                        *level_selection = LevelSelection::Uid(ldtk_level.level.uid);
                    }
                }
            }
        }
    }
}
