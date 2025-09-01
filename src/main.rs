use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Linda Gennaro" {
            name.0 = "Linda Belcher".to_string();
            break;
        }
    }
}
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bob Belcher".to_string())));
    commands.spawn((Person, Name("Linda Gennaro".to_string())));
    commands.spawn((Person, Name("Tina Belcher".to_string())));
    commands.spawn((Person, Name("Gene Belcher".to_string())));
    commands.spawn((Person, Name("Louise Belcher".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
        println!(" == DONE WITH GREETINGS == ");
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
