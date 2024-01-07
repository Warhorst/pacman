use bevy::prelude::*;

pub(super) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HighScoreWasBeaten>()
            .register_type::<Score>()
            .register_type::<HighScore>()
            .register_type::<HighScoreWasBeaten>()
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

/// Stores the current high score of the game.
#[derive(Resource, Reflect)]
pub struct HighScore {
    /// The actual high score
    pub score: usize,
    /// Tells if the high score was beaten in the current game. This is necessary to tell if the player
    /// has just beaten the score or if the player broke it and continues to collect points.
    pub was_beaten: bool
}

impl HighScore {
    pub fn new(score: usize) -> Self {
        HighScore {
            score,
            was_beaten: false
        }
    }
}

/// Fired when the player broke the current high score
#[derive(Event, Reflect)]
pub struct HighScoreWasBeaten;

/// Identifies floating text which pops up when pacman ate a ghost.
#[derive(Component, Reflect)]
pub struct ScoreText;

/// Tells how long a floating score text remains visible before it disappears.
#[derive(Component, Reflect, Deref, DerefMut)]
pub struct ScoreTextTimer(pub Timer);

/// Keeps track of how many ghosts pacman ate during an active energizer
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct EatenGhostCounter(pub usize);