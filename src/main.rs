pub mod tile;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use tile::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), TilePlugin))
    .run();
}