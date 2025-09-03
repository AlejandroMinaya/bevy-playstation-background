use bevy::prelude::*;

const FREQUENCY: f64 = 120.0;

const PARTICLE_SIZE: f32 = 2.0;
const TOTAL_PARTICLES: i32 = 360;
const AMPLITUDE: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(FREQUENCY))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (advance_all_start_timers, move_particle).chain(),
        )
        .run();
}

#[derive(Component)]
struct StartTimer(Timer);

fn advance_all_start_timers(time: Res<Time>, mut query: Query<(&mut StartTimer, &mut Particle)>) {
    for (mut timer, mut particle) in &mut query {
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
/*
* y = x or sin(x)
* x = time_elapsed
* dy = transform.translation.y
* dx = 1/FREQUENCY
*/

fn y(x: f32) -> f32 {
    x.sin()
}
fn move_particle(time: Res<Time>, mut query: Query<(&mut Transform, &StartTimer, &Particle)>) {
    for (mut transform, start_timer, particle) in &mut query {
        if start_timer.0.finished() {
            let x = time.elapsed_secs() - particle.start_time;
            println!("X: {}", x);
            transform.translation.y = y(x) * AMPLITUDE;
            println!("Translation Y: {}", transform.translation.y);
            /*
            match *direction {
                Direction::Up => transform.translation.y = x.sin(),
                Direction::Down => transform.translation.y = (-x).sin(),
            }

            if transform.translation.y >= AMPLITUDE {
                *direction = Direction::Down;
            } else if transform.translation.y <= -AMPLITUDE {
                *direction = Direction::Up;
            }
            */
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
        let x_pos = PARTICLE_SIZE * id as f32;
        commands.spawn((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, 0.0, 0.0),
            StartTimer(Timer::from_seconds(
                (1_f32 / FREQUENCY as f32) * id as f32,
                TimerMode::Once,
            )),
            Particle::default(),
        ));
    }
}
