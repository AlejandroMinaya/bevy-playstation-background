use bevy::prelude::*;

const FPS: f64 = 120.0;

const TOTAL_PARTICLES: i32 = 500;

const X_ORIGIN: f32 = -TOTAL_PARTICLES as f32 / 2.0;

#[derive(Resource, Clone)]
struct WaveConfig {
    wavelength: f32,
    speed: f32,
    amplitude: f32,
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
            amplitude: 100.0,
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
fn move_particle(
    wave_config: Res<WaveConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &StartTimer, &Particle)>,
) {
    for (mut transform, start_timer, particle) in &mut query {
        if start_timer.0.finished() {
            let x = time.elapsed_secs() - particle.start_time;
            transform.translation.y = y(x * wave_config.frequency()) * wave_config.amplitude;
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
        let circle = meshes.add(Rectangle::new(1.0, 1.0));
        let x_pos = X_ORIGIN + id as f32;
        commands.spawn((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, 0.0, 0.0),
            StartTimer(Timer::from_seconds(
                1.0 / wave_config.speed * id as f32,
                TimerMode::Once,
            )),
            Particle::default(),
        ));
    }
}
