use std::time::Duration;
use bevy::prelude::*;

pub(super) struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SoundEffect>()
        ;
    }
}

/// Marker for a sound effect that gets played when certain things happen, like pacman
/// eating dots. Has a timer on it to despawn it later.
#[derive(Component, Reflect)]
pub struct SoundEffect {
    timer: Timer
}

impl SoundEffect {
    /// Create a new sound effect with a timer of one second.
    pub fn new(duration_secs: u64) -> Self {
        SoundEffect {
            timer: Timer::new(Duration::from_secs(duration_secs), TimerMode::Once)
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}