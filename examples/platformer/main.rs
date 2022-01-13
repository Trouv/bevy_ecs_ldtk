use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use bevy::render::{options::WgpuOptions, render_resource::WgpuLimits};

use heron::prelude::*;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            limits: WgpuLimits {
                max_texture_array_layers: 2048,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -2000., 0.0)))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
        })
        .add_startup_system(setup)
        .add_system(pause_physics_during_load)
        .add_system(movement)
        //.add_system(detect_climb_range)
        //.add_system(ignore_gravity_if_climbing)
        .add_system(patrol)
        .add_system(camera_fit_inside_current_level)
        //.add_system(debug_collision)
        .add_system(update_level_selection)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<LadderBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<MobBundle>("Mob")
        .register_ldtk_entity::<ChestBundle>("Chest")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

fn pause_physics_during_load(
    mut level_events: EventReader<LevelEvent>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for event in level_events.iter() {
        println!("{:?}", event);

        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct ColliderBundle {
    collider: CollisionShape,
    rigid_body: RigidBody,
    velocity: Velocity,
    rotation_constraints: RotationConstraints,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(6., 16., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(5., 5., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        match int_grid_cell.value {
            1 => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Static,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            2 => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints: RotationConstraints::lock(),
                ..Default::default()
            },
            3 => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Static,
                rotation_constraints: RotationConstraints::lock(),
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
    #[ldtk_entity]
    worldly: Worldly,
    climber: Climber,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct Climbable;

impl Default for Climber {
    fn default() -> Climber {
        Climber::OutOfClimbRange
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct WallBundle {
    #[from_int_grid_cell]
    #[bundle]
    collider_bundle: ColliderBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
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

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "patrol".to_string())
            .unwrap();
        if let FieldValue::Points(ldtk_points) = &ldtk_patrol.value {
            for ldtk_point in ldtk_points {
                if let Some(ldtk_point) = ldtk_point {
                    // The +1 is necessary here due to the pivot of the entities in the sample
                    // file.
                    // The patrols set up in the file look flat and grounded,
                    // but technically they're not if you consider the pivot,
                    // which is at the bottom-center for the skulls.
                    let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                        * Vec2::splat(layer_instance.grid_size as f32);

                    points.push(ldtk_pixel_coords_to_translation_pivoted(
                        pixel_coords.as_ivec2(),
                        layer_instance.c_hei * layer_instance.grid_size,
                        IVec2::new(entity_instance.width, entity_instance.height),
                        entity_instance.pivot,
                    ));
                }
            }
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
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
    #[from_entity_instance]
    entity_instance: EntityInstance,
    #[ldtk_entity]
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
    mut query: Query<(&mut Velocity, &mut Climber), With<Player>>,
) {
    for (mut velocity, mut climber) in query.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linear.x = (right - left) * 200.;

        if *climber == Climber::InClimbRange
            && (input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::S))
        {
            *climber = Climber::Climbing;
        }

        if *climber == Climber::Climbing {
            let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

            velocity.linear.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) {
            velocity.linear.y = 450.;
            *climber = Climber::InClimbRange;
        }
    }
}

//fn detect_climb_range(
//mut climbers: Query<(Entity, &mut Climber)>,
//climbables: Query<&Climbable>,
//mut collisions: EventReader<CollisionEvent>,
//) {
//let mut climbers_in_range = HashSet::new();
//for collision in collisions.iter() {
//if climbers.get_mut(collision.entity).is_ok()
//&& climbables
//.get_component::<Climbable>(collision.other_entity)
//.is_ok()
//{
//climbers_in_range.insert(collision.entity);
//}
//}

//for (entity, mut climber) in climbers.iter_mut() {
//if !climbers_in_range.contains(&entity) {
//*climber = Climber::OutOfClimbRange;
//} else if *climber != Climber::Climbing {
//*climber = Climber::InClimbRange;
//}
//}
//}

//fn ignore_gravity_if_climbing(
//mut commands: Commands,
//query: Query<(Entity, &Climber), Changed<Climber>>,
//) {
//for (entity, climber) in query.iter() {
//match *climber {
//Climber::Climbing => {
//commands.entity(entity).insert(IgnoreGravity);
//}
//_ => {
//commands.entity(entity).remove::<IgnoreGravity>();
//}
//}
//}
//}

fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
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

fn camera_fit_inside_current_level(
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

fn update_level_selection(
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
