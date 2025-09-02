use bevy::prelude::*;

const PARTICLE_SIZE: f32 = 10.0;
const TOTAL_PARTICLES: i32 = 10;

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
        transform.translation.y += 20. * time.delta_secs()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::oklaba(1.0, 0.0, 0.0, 1.0);

    commands.spawn(Camera2d);

    for id in 0..=TOTAL_PARTICLES {
        let circle = meshes.add(Rectangle::new(PARTICLE_SIZE, PARTICLE_SIZE));
        let x_pos = PARTICLE_SIZE * id as f32;
        commands.spawn((
            Particle,
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, 0.0, 0.0),
        ));
    }
}
