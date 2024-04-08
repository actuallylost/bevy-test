mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;

use bevy::prelude::*;
use state::StatePlugin;

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
            StatePlugin,
            AssetLoaderPlugin,
            SchedulePlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
            MovementPlugin,
            SpaceshipPlugin,
            AsteroidsPlugin,
            CameraPlugin,
            // DebugPlugin,
        ))
        .run();
}
