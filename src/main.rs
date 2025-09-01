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

fn hello_world() {
    println!("Hello World!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
