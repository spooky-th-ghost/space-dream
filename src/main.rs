use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
pub enum PlayerAction {
    #[default]
    RearRightThruster,
    RearLeftThruster,
    FrontRightThruster,
    FrontLeftThruster,
    BottomRightThruster,
    BottomLeftThruster,
    TopRightThruster,
    TopLeftThruster,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map = leafwing_input_manager::input_map::InputMap::new([
            (KeyCode::I, RearRightThruster),
            (KeyCode::R, RearLeftThruster),
            (KeyCode::Apostrophe, FrontRightThruster),
            (KeyCode::A, FrontLeftThruster),
            (KeyCode::O, BottomRightThruster),
            (KeyCode::E, BottomLeftThruster),
            (KeyCode::P, TopRightThruster),
            (KeyCode::W, TopLeftThruster),
        ])
        .build();

        InputListenerBundle {
            input_manager: InputManagerBundle {
                input_map,
                ..default()
            },
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, handle_thrusters)
        .run()
}

// QWOP style
// Steer a spaceship while having to use Q,W,E,I,O,P and Space
// dreamy cute stuff
// could do a roguelike
#[derive(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::default(),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(InputListenerBundle::input_map())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 7.0),
                ..default()
            });
        });

    for _ in 0..100 {
        let x = rand::thread_rng().gen_range(-100.0..100.0);
        let y = rand::thread_rng().gen_range(-100.0..100.0);
        let z = rand::thread_rng().gen_range(-100.0..100.0);
        let red = rand::thread_rng().gen_range(0.0..1.0);
        let green = rand::thread_rng().gen_range(0.0..1.0);
        let blue = rand::thread_rng().gen_range(0.0..1.0);
        let size = rand::thread_rng().gen_range(1.0..10.0);
        let color = Color::Rgba {
            red,
            green,
            blue,
            alpha: 1.0,
        };

        let transform = Transform::from_xyz(x, y, z);
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: size,
                    sectors: 8,
                    stacks: 8,
                })),
                material: materials.add(color.into()),
                transform,
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(Collider::capsule_y(0.5, 0.5));
    }
}

fn handle_thrusters(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Velocity, &Transform, &ActionState<PlayerAction>)>,
) {
    for (mut velocity, transform, action) in &mut player_query {
        let mut angular_velocity_to_add = Vec3::ZERO;
        let mut linear_velocity_to_add = Vec3::ZERO;

        let forward_vector = transform.forward();
        let up_vector = transform.up();
        let right_vector = transform.right();

        if action.pressed(PlayerAction::RearRightThruster)
            && action.pressed(PlayerAction::RearLeftThruster)
        {
            linear_velocity_to_add += forward_vector * 0.2 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::FrontRightThruster)
            && action.pressed(PlayerAction::FrontLeftThruster)
        {
            linear_velocity_to_add += forward_vector * -0.2 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::BottomRightThruster)
            && action.pressed(PlayerAction::BottomLeftThruster)
        {
            linear_velocity_to_add += up_vector * 0.2 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::TopRightThruster)
            && action.pressed(PlayerAction::TopLeftThruster)
        {
            linear_velocity_to_add += up_vector * -0.2 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::RearRightThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(0.3, 0.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += up_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::RearLeftThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(-0.3, 0.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += up_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::FrontRightThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(0.3, 0.0, -0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += up_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::FrontLeftThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(-0.3, 0.0, -0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += up_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::BottomRightThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(0.3, -1.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += forward_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::BottomLeftThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(-0.3, -1.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += forward_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::TopRightThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(0.3, 1.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += forward_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::TopLeftThruster) {
            gizmos.sphere(
                transform.translation + Vec3::new(-0.3, 1.0, 0.5),
                transform.rotation,
                0.5,
                Color::ORANGE,
            );
            angular_velocity_to_add += forward_vector * 0.05 * time.delta_seconds();
        }

        if angular_velocity_to_add != Vec3::ZERO {
            velocity.angvel += angular_velocity_to_add;
        }

        if linear_velocity_to_add != Vec3::ZERO {
            velocity.linvel += linear_velocity_to_add;
        }
    }
}
