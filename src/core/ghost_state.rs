use bevy::prelude::*;

pub(super) struct GhostStatePlugin;

impl Plugin for GhostStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GhostState>()
        ;
    }
}

/// The current state of a ghost
#[derive(Component, Reflect, Copy, Clone, Debug, Eq, PartialEq)]
pub enum GhostState {
    /// Move to the ghost corner
    Scatter,
    /// Chase pacman with your matching chase technique
    Chase,
    /// Run around aimlessly until the active energizer ends
    Frightened,
    /// Return to the ghost house to respawn
    Eaten,
    /// Leave the ghost house after respawning
    Spawned,
}