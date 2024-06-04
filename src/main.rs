use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Murilo Ijanc" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ana Luisa".to_string())));
    commands.spawn((Person, Name("Murilo Ijanc".to_string())));
}

fn hello_world() {
    println!("hello world!");
}
