use bevy::prelude::*;

const PARTICLE_SIZE: f32 = 10.0;
const TOTAL_PARTICLES: i32 = 10;
const STAGGER_DELAY_SCS: f32 = 0.25;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (advance_start_timers, move_particle).chain())
        .run();
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct StartTimer(Timer);

fn move_particle(time: Res<Time>, mut query: Query<(&mut Transform, &StartTimer), With<Particle>>) {
    for (mut transform, start_timer) in &mut query {
        if start_timer.0.finished() {
            transform.translation.y += 20. * time.delta_secs();
        }
    }
}
fn advance_start_timers(time: Res<Time>, mut query: Query<&mut StartTimer>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
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
            StartTimer(Timer::from_seconds(
                STAGGER_DELAY_SCS * id as f32,
                TimerMode::Once,
            )),
        ));
    }
}
