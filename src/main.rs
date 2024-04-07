use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
