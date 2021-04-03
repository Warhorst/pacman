use bevy::prelude::*;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<DotEaten>()
            .add_event::<PacmanKilled>()
            .add_event::<GhostPassedTunnel>()
            .add_event::<EnergizerEaten>();
    }
}

/// Fired when pacman eats a dot.
pub struct DotEaten;

/// Fired when pacman eats an energizer.
pub struct EnergizerEaten;

/// Fired when pacman was killed by a ghost.
pub struct PacmanKilled;

/// Fired when a ghost moved through a tunnel.
/// Saves the entity of the ghost.
pub struct GhostPassedTunnel {
    pub entity: Entity
}