use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_world)
        .add_systems(Update, spawn_life);
    }
}

// Components
#[derive(Component, Default, Clone, Copy)]
pub struct IsAlive(bool);

#[derive(Component, Default, Clone, Copy)]
pub struct IsNearLife(bool);

#[derive(Component, Default, Clone, Copy)]
pub struct HasLife(bool);

#[derive(Component, Default, Clone, Copy)]
pub struct Tile {
    pub x: u16,
    pub y: u16
}

// Bundles
#[derive(Bundle, Default, Clone, Copy)]
pub struct TileBundle {
    pub is_alive: IsAlive,
    pub is_near_life: IsNearLife,
    pub position: Tile
}

impl TileBundle {
    fn new(is_alive: IsAlive, is_near_life: IsNearLife, position: Tile) -> Self {
        Self {
            is_alive,
            is_near_life,
            position
        }
    }

    fn _get_is_alive(&self) -> IsAlive {
        self.is_alive
    }

    fn _get_is_near_life(&self) -> IsNearLife {
        self.is_near_life
    }

    fn _get_pos(&self) -> Tile {
        self.position
    }
}

// Create 2D plane
fn create_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Create grid
    todo!("Create grid");
}


// Spawn life into the world
fn spawn_life(mut commands: Commands, time: Res<Time>, keys: Res<Input<KeyCode>>) {
    let life = TileBundle::new(IsAlive(true), IsNearLife(false), Tile::default());
    
    let space = keys.pressed(KeyCode::Space);

    if space {
        spawn_life;
    }
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