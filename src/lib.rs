use bevy::prelude::*;

pub struct WavePlugin;

const FPS: f64 = 120.0;

const WAVELENGTH: f32 = 100.0;
const SPEED: f32 = 3000.0;
const AMPLITUDE: f32 = 100.0;
const FREQUENCY: f32 = SPEED / WAVELENGTH;

const TOTAL_PARTICLES: i32 = 500;
const PARTICLE_SIZE: f32 = 1.0;

const X_ORIGIN: f32 = -TOTAL_PARTICLES as f32 * PARTICLE_SIZE / 2.0;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(FPS));
        app.add_systems(Startup, setup);
        app.add_systems(
            FixedUpdate,
            (advance_all_start_timers, move_particle).chain(),
        );
    }
}

#[derive(Component)]
struct StartTimer(Timer);

fn advance_all_start_timers(time: Res<Time>, mut query: Query<(&mut StartTimer, &mut Particle)>) {
    for (mut timer, mut particle) in &mut query {
        if timer.0.finished() {
            continue;
        }

        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            particle.start_time = time.elapsed_secs();
        }
    }
}

#[derive(Component, Default)]
struct Particle {
    start_time: f32,
}

fn y(x: f32) -> f32 {
    x.sin()
}
fn move_particle(time: Res<Time>, mut query: Query<(&mut Transform, &StartTimer, &Particle)>) {
    for (mut transform, start_timer, particle) in &mut query {
        if start_timer.0.finished() {
            let x = time.elapsed_secs() - particle.start_time;
            transform.translation.y = y(x * FREQUENCY) * AMPLITUDE;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::oklaba(1.0, 0.0, 0.0, 1.0);

    commands.spawn(Camera2d);

    for id in 0..TOTAL_PARTICLES {
        let circle = meshes.add(Rectangle::new(PARTICLE_SIZE, PARTICLE_SIZE));
        let x_pos = X_ORIGIN + PARTICLE_SIZE * id as f32;
        commands.spawn((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, 0.0, 0.0),
            StartTimer(Timer::from_seconds(
                PARTICLE_SIZE / SPEED * id as f32,
                TimerMode::Once,
            )),
            Particle::default(),
        ));
    }
}
