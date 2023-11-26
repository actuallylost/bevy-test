mod plugins;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;


fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), LifePlugin, CameraPlugin))
    .run();
}