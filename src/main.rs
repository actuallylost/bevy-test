use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

use rand::random;

// Components
#[derive(Component)]
pub struct State(bool);
#[derive(Component)]
pub struct Position{
    x: f32,
    y: f32
}

// Bundles
#[derive(Bundle)]
struct LifeBundle {
    pub state: State,
    pub position: Position
}

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, EditorPlugin::default(), NoCameraPlayerPlugin))
    .add_systems(Startup, (setup_cam, create_world, spawn_cubes))
    .add_systems(Update, spawn_life)
    .run();
}

// TODO: Remove camera when game of life is implemented
fn setup_cam(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}

// Create 2d plane
fn create_world(mut _commands: Commands) {
    todo!("Implement world building");
}


// Spawns life into the world
fn spawn_life(mut commands: Commands) {
    let life = LifeBundle {
        state: State(random::<bool>()),
            position: Position{x: random::<f32>(), y: random::<f32>()}
        };
            
    commands.spawn(life);
}

fn spawn_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.).into());
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
                ..Default::default()
            });
        }
    }
}