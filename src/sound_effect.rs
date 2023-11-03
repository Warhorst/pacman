use bevy::prelude::*;
use std::time::Duration;

pub(super)  struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SoundEfect>()
            .add_systems(
                Update,
                update_sound_effects
            )
        ;
    }
}

/// Marker for a sound effect that gets played when certain things happen, like pacman
/// eating dots. Has a timer on it to despawn it later.
#[derive(Component, Reflect)]
pub struct SoundEfect {
    timer: Timer
}

impl SoundEfect {
    /// Create a new sound effect with a timer of one second.
    pub fn new() -> Self {
        SoundEfect {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }

    fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    fn finished(&self) -> bool {
        self.timer.finished()
    }
}

/// Updates the timer on a all sound effects. As I currently know no other way to check if a sound
/// finished playing, this is the solution.
/// It doesn't matter if the sound plays longer than one second, the entity can be deleted
/// anyway without interrupting it.
fn update_sound_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut sounds: Query<(Entity, &mut SoundEfect)>
) {
    let delta = time.delta();

    for (entity, mut sound) in &mut sounds {
        sound.update(delta);

        if sound.finished() {
            commands.entity(entity).despawn();
        }
    }
}