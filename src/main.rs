use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

use rand::random;

// Components
#[derive(Component)]
struct IsAlive(bool);

impl Default for IsAlive {
    fn default() -> Self {
        IsAlive(true)
    }
}

#[derive(Component)]
struct IsNearLife(bool);

impl Default for IsNearLife {
    fn default() -> Self {
        IsNearLife(false)
    }
}

#[derive(Component)]
struct HasLife(bool);

impl Default for HasLife {
    fn default() -> Self {
        HasLife(false)
    }
}

#[derive(Component)]
struct Tile {
    x: u16,
    y: u16
}

// Bundles
#[derive(Bundle)]
struct LifeBundle {
    pub is_alive: IsAlive,
    pub is_near_life: IsNearLife,
    pub position: Tile
}

impl Default for LifeBundle {
    fn default() -> Self {
        LifeBundle {
            is_alive: IsAlive::default(),
            is_near_life: IsNearLife::default(),
            position: Tile{x: random::<u16>(), y: random::<u16>()}
        }
    }
}

#[derive(Bundle)]
struct TileBundle {
    pub has_life: HasLife,
    pub position: Tile
}


fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), NoCameraPlayerPlugin))
    .add_systems(Startup, create_world)
    .add_systems(Update, spawn_life)
    .run();
}

// Create 2D plane
fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Create grid
    // commands.spawn(NodeBundle {
    //     style: Style {
    //         display: Display::Grid,
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //     },
    //     background_color: BackgroundColor(Color::WHITE),
    //     ..default()
    // });
}


// Spawn life into the world
fn spawn_life(mut commands: Commands) {
    let life = LifeBundle::default();
            
    commands.spawn(life);
}

// fn spawn_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
//     let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());
//     for x in -10..10 {
//         for z in -10..10 {
//             commands.spawn(PbrBundle {
//                 mesh: mesh.clone(),
//                 transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
//                 ..Default::default()
//             });
//         }
//     }
// }