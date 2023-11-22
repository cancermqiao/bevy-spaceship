use bevy::prelude::*;

use crate::movement::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform, &Velocity)>) {
    for (entity, transform, _) in query.iter() {
        info!(
            "Entity {:?} is at position {:?}",
            entity, transform.translation
        );
    }
}
