use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Name1".to_owned())));
    commands.spawn((Person, Name("Name2".to_owned())));
    commands.spawn((Person, Name("Name3".to_owned())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Name1" {
            name.0 = "Name4".to_owned();
            break;
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            // Chaining tells the systems to run in the exact order they're listed
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
