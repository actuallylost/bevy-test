mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
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
            AssetLoaderPlugin,
            CollisionDetectionPlugin,
            MovementPlugin,
            SpaceshipPlugin,
            AsteroidsPlugin,
            CameraPlugin,
            // DebugPlugin,
        ))
        .run();
}
