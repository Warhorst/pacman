use bevy::prelude::*;

/// Configures the system sets of the game, defining their order of execution.
pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, DetectIntersectionsWithPacman.before(ProcessIntersectionsWithPacman))
            .configure_sets(Update, ProcessIntersectionsWithPacman.before(SetState))
            .configure_sets(Update, SetState.before(SetTarget))
            .configure_sets(Update, SetTarget.before(MoveEntities))
            .configure_sets(Update, MoveEntities.before(UpdateGameState))
        ;
    }
}

// Set for all systems that set the state of a ghost.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetState;

/// Set for all systems that set the target of a ghost.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetTarget;

/// Set for all systems that move movable entities around the map.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MoveEntities;

/// Set for all systems that detect intersections between pacman and other entities.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DetectIntersectionsWithPacman;

/// Set for all systems that process intersections between pacman and other entities.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessIntersectionsWithPacman;

/// Set for all systems which update the current game state
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateGameState;