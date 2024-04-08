use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    // Log entity ID and Position of each entity with the "Position" component
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}
