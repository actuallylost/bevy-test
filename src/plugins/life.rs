use bevy::prelude::*;

struct LifePlugin;
const PLGN_NAME: &str = "LifePlugin"; 
    
impl Plugin for LifePlugin {
    fn build(&self, _app: &mut App) {
        todo!("Implement")
    }

    fn name(&self) -> &str {
        PLGN_NAME
    }

    fn ready(&self, _app: &App) -> bool {
        todo!("Implement")
    }

    fn finish(&self, _app: &mut App) {
        todo!("Implement")
    }

    fn cleanup(&self, _app: &mut App) {
        todo!("Implement")
   }
}
