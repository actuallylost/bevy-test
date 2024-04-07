mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins((
            DefaultPlugins,
            // Game's Plugins
            SpaceshipPlugin,
            AsteroidsPlugin,
            CameraPlugin,
            MovementPlugin,
            DebugPlugin,
        ))
        .run();
}
