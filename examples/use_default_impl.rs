use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_X_VELOCITY: f32 = 5.0;

/* Components */

#[derive(Component)]
pub struct GameCamera;

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
#[bevy_ecs_ldtk(use_default_impl)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(6., 14.),
            controller: default(),
            sprite_bundle: default(),
            player: default(),
        }
    }
}

#[derive(Default, Component)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
#[bevy_ecs_ldtk(use_default_impl)]
pub struct WallBundle {
    pub wall: Wall,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(8., 8.),
            wall: default()
        }
    }
}

/* Components */

/* Systems */

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        GameCamera
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("use_default_impl.ldtk"),
        ..Default::default()
    });
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y - ((player_transform.translation.y / 5.) * 4.);
}

pub fn movement(
    keycode: Res<Input<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>
) {
    let Ok(mut player_controller) = player_query.get_single_mut() else { return; };
    let mut translation = Vec2::default();

    if keycode.pressed(KeyCode::D) || keycode.pressed(KeyCode::Right) {
        translation.x += PLAYER_X_VELOCITY;
    }

    if keycode.pressed(KeyCode::A) || keycode.pressed(KeyCode::Left) {
        translation.x -= PLAYER_X_VELOCITY;
    }

    player_controller.translation = Some(translation);
}

/* Systems */

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            LdtkPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (follow_player, movement))
        .register_ldtk_int_cell_for_layer::<WallBundle>("IntGrid", 1)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run()
}
