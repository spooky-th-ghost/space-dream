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

impl PlayerAction {
    pub fn thruster_iterator() -> impl Iterator<Item = PlayerAction> {
        use PlayerAction::*;
        [
            RearRightThruster,
            RearLeftThruster,
            FrontRightThruster,
            FrontLeftThruster,
            BottomRightThruster,
            BottomLeftThruster,
            TopRightThruster,
            TopLeftThruster,
        ]
        .iter()
        .copied()
    }
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

#[derive(Component)]
pub struct Thruster(pub PlayerAction);

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
        .insert(Player)
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 7.0),
                ..default()
            });

            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0.3, 0.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::RearRightThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(-0.3, 0.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::RearLeftThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0.3, 0.0, -0.5),
                    ..default()
                },
                Thruster(PlayerAction::FrontRightThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(-0.3, 0.0, -0.5),
                    ..default()
                },
                Thruster(PlayerAction::FrontLeftThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0.3, -1.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::BottomRightThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(-0.3, -1.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::BottomLeftThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0.3, 1.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::TopRightThruster),
                RigidBody::KinematicPositionBased,
            ));
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(-0.3, 1.0, 0.5),
                    ..default()
                },
                Thruster(PlayerAction::TopLeftThruster),
                RigidBody::KinematicPositionBased,
            ));
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
    mut player_query: Query<(&mut Velocity, &Transform, &ActionState<PlayerAction>), With<Player>>,
    thruster_query: Query<(&Transform, &Thruster), Without<Player>>,
) {
    for (mut velocity, transform, action) in &mut player_query {
        let mut angular_velocity_to_add = Vec3::ZERO;
        let mut linear_velocity_to_add = Vec3::ZERO;

        let forward_vector = transform.forward();
        let up_vector = transform.up();

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

        // Handles Thruster Gizmos
        for thruster in PlayerAction::thruster_iterator() {
            for (thruster_transform, thruster_component) in &thruster_query {
                if action.pressed(thruster) && thruster_component.0 == thruster {
                    gizmos.sphere(
                        thruster_transform.translation,
                        thruster_transform.rotation,
                        0.5,
                        Color::ORANGE,
                    );
                    if thruster == PlayerAction::RearRightThruster {
                        println!("{:?}:{}", thruster, thruster_transform.translation);
                    }
                }
            }
        }

        if action.pressed(PlayerAction::RearRightThruster) {
            angular_velocity_to_add += up_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::RearLeftThruster) {
            angular_velocity_to_add += up_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::FrontRightThruster) {
            angular_velocity_to_add += up_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::FrontLeftThruster) {
            angular_velocity_to_add += up_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::BottomRightThruster) {
            angular_velocity_to_add += forward_vector * 0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::BottomLeftThruster) {
            angular_velocity_to_add += forward_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::TopRightThruster) {
            angular_velocity_to_add += forward_vector * -0.05 * time.delta_seconds();
        }

        if action.pressed(PlayerAction::TopLeftThruster) {
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
