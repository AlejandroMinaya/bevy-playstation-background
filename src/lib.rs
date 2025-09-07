use std::f32::consts::PI;

use bevy::prelude::*;

const FPS: f64 = 120.0;

const TOTAL_PARTICLES: i32 = 500;

const X_ORIGIN: f32 = -TOTAL_PARTICLES as f32 / 2.0;

const WAVELENGTH_STEP: f32 = 10.0;
const SPEED_STEP: f32 = 10.0;
const AMPLITUDE_STEP: f32 = 10.0;

#[derive(Resource, Clone)]
struct WaveConfig {
    wavelength: f32,
    speed: f32,
    amplitude: f32,
    height: f32,
}
impl WaveConfig {
    fn frequency(&self) -> f32 {
        self.speed / self.wavelength
    }
}
impl Default for WaveConfig {
    fn default() -> Self {
        WaveConfig {
            wavelength: 100.0,
            speed: 100.0,
            amplitude: 50.0,
            height: 450.0,
        }
    }
}

#[derive(Default)]
pub struct WavePlugin {
    initial_config: WaveConfig,
}

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(FPS));
        app.insert_resource(self.initial_config.clone());
        app.add_systems(Startup, setup);
        app.add_systems(
            FixedUpdate,
            (advance_all_start_timers, move_particle).chain(),
        );
        app.add_systems(Update, keyboard_input_system);
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
    x.sin() * (x * PI).cos()
}
fn move_particle(
    wave_config: Res<WaveConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &StartTimer, &Particle)>,
) {
    for (mut transform, start_timer, particle) in &mut query {
        if start_timer.0.finished() {
            let x = time.elapsed_secs() - particle.start_time;
            transform.translation.y =
                y(x * wave_config.frequency()) * wave_config.amplitude - wave_config.height;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    wave_config: Res<WaveConfig>,
) {
    let color = Color::oklaba(1.0, 0.0, 0.0, 1.0);

    commands.spawn(Camera2d);

    for id in 0..TOTAL_PARTICLES {
        let circle = meshes.add(Rectangle::new(1.0, wave_config.height));
        let x_pos = X_ORIGIN + id as f32;
        commands.spawn((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, -wave_config.height, 0.0),
            StartTimer(Timer::from_seconds(
                1.0 / wave_config.speed * id as f32,
                TimerMode::Once,
            )),
            Particle::default(),
        ));
    }
}

fn keyboard_input_system(
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut wave_config: ResMut<WaveConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        wave_config.amplitude += AMPLITUDE_STEP;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        wave_config.amplitude -= AMPLITUDE_STEP;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        wave_config.wavelength += WAVELENGTH_STEP;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        wave_config.wavelength -= WAVELENGTH_STEP;
    }
    if keyboard_input.just_pressed(KeyCode::Period) {
        wave_config.speed += SPEED_STEP;
    }
    if keyboard_input.just_pressed(KeyCode::Comma) {
        wave_config.speed -= SPEED_STEP;
    }
}
