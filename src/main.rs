pub mod life;
pub mod camera;

// Bevy
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use life::LifePlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), LifePlugin, CameraPlugin))
    .run();
}