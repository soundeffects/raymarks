use bevy::prelude::*;
use rand::Rng;

const CUBE_COUNT: usize = 1000;

// Component to mark entities that should be cleaned up
#[derive(Component)]
struct Temporary;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_random_cubes, cleanup_cubes))
        .run();
}

fn setup(mut commands: Commands) {
    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add a light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn spawn_random_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    // Spawn 5 random cubes
    for _ in 0..CUBE_COUNT {
        let size = Vec3::new(
            rng.random_range(0.5..2.0), // x
            rng.random_range(0.5..2.0), // y
            rng.random_range(0.5..2.0), // z
        );

        let color = Color::srgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        );

        let position = Vec3::new(
            rng.random_range(-5.0..5.0), // x
            rng.random_range(0.0..5.0),  // y
            rng.random_range(-5.0..5.0), // z
        );

        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::from_size(size)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(position),
            Temporary, // Mark for cleanup
        ));
    }
}

fn cleanup_cubes(mut commands: Commands, query: Query<Entity, With<Temporary>>) {
    // Remove all entities marked as Temporary
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
