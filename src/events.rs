use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<DotEatenEvent>()
            .add_event::<PacmanKilledEvent>();
    }
}

/// Fired when pacman eats a dot.
/// Contains a reference to the eaten dot.
pub struct DotEatenEvent;

/// Fired when pacman was killed by a ghost.
pub struct PacmanKilledEvent;