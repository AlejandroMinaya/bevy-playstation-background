use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_particle)
        .run();
}

#[derive(Component)]
struct Particle;

fn move_particle(time: Res<Time>, mut query: Query<&mut Transform, With<Particle>>) {
    for mut transform in &mut query {
        transform.translation.x += 20. * time.delta_secs()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let circle = meshes.add(Circle::new(10.0));
    let color = Color::oklaba(1.0, 0.0, 0.0, 1.0);

    commands.spawn((
        Particle,
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
