use bevy::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, (setup_cam, spawn_cubes));
    }
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
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