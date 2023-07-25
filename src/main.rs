use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        .add_systems(Startup, setup)
        .run()
}

// QWOP style
// Steer a spaceship while having to use Q,W,E,I,O,P and Space
// dreamy cute stuff
// could do a roguelike

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
            transform: Transform::default()
                .with_rotation(Quat::from_axis_angle(Vec3::X, 90.0f32.to_radians())),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::capsule_y(0.5, 0.5))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, -2.0, -5.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::X, 90.0f32.to_radians())),
                ..default()
            });
        });
}
