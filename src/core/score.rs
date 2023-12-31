use bevy::prelude::*;

pub(super) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Score>()
            .register_type::<ScoreText>()
            .register_type::<ScoreTextTimer>()
            .register_type::<EatenGhostCounter>()
        ;
    }
}

/// Resource that saves how many points the player has collected so far
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct Score(pub usize);

impl Score {
    pub fn add(&mut self, points: usize) {
        **self += points
    }
}

/// Identifies floating text which pops up when pacman ate a ghost.
#[derive(Component, Reflect)]
pub struct ScoreText;

/// Tells how long a floating score text remains visible before it disappears.
#[derive(Component, Reflect, Deref, DerefMut)]
pub struct ScoreTextTimer(pub Timer);

/// Keeps track of how many ghosts pacman ate during an active energizer
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct EatenGhostCounter(pub usize);