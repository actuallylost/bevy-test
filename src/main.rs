mod plugins;

// Bevy
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
// Plugins
use plugins::life::*;
use plugins::camera::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), LifePlugin, CameraPlugin))
    .run();
}