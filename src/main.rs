use bevy::prelude::*;

const MAX_FPS: f64 = 120.0;
const MIN_PERIOD: f32 = 1.0 / MAX_FPS as f32;

const PARTICLE_SIZE: f32 = 2.0;
const PARTICLE_SPEED: f32 = 1.0;
const TOTAL_PARTICLES: i32 = 300;
const FREQUENCY: f32 = 100.0;
const PERIOD: f32 = 1.0 / FREQUENCY;
const AMPLITUDE: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(MAX_FPS))
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

fn advance_all_start_timers(time: Res<Time>, mut query: Query<&mut StartTimer>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn move_particle(mut query: Query<(&mut Transform, &mut Direction, &StartTimer)>) {
    for (mut transform, mut direction, start_timer) in &mut query {
        if start_timer.0.finished() {
            match *direction {
                Direction::Up => transform.translation.y += PARTICLE_SPEED,
                Direction::Down => transform.translation.y -= PARTICLE_SPEED,
            }

            if transform.translation.y >= AMPLITUDE {
                *direction = Direction::Down;
            } else if transform.translation.y <= -AMPLITUDE {
                *direction = Direction::Up;
            }
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
            Direction::Up,
            Transform::from_xyz(x_pos, 0.0, 0.0),
            StartTimer(Timer::from_seconds(
                PERIOD.clamp(MIN_PERIOD, 1.0) * id as f32,
                TimerMode::Once,
            )),
        ));
    }
}
