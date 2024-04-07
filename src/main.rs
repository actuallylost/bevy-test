mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SpaceshipPlugin, MovementPlugin, DebugPlugin))
        .run();
}
