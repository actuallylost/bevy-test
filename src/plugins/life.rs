use bevy::prelude::*;
use rand::random;

struct LifePlugin;

// Components
#[derive(Component)]
pub struct State(bool);
#[derive(Component)]
pub struct Position{x: f32, y: f32}

// Bundles
#[derive(Bundle)]
struct LifeBundle {
    pub state: State,
    pub position: Position
}

// Constants
const PLGN_NAME: &str = "LifePlugin"; 
    
impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_life);
    }

    fn name(&self) -> &str {
        PLGN_NAME
    }
}

// Spawns life into the world
fn spawn_life(mut cmd: Commands) {
    let life = LifeBundle {
        state: State(random::<bool>()),
        position: Position{ x: random::<f32>(), y: random::<f32>() }
    };

    cmd.spawn(life);
}
