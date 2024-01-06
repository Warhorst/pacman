use bevy::prelude::*;
use crate::core::prelude::*;

pub(super) struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                update_sound_effects
            )
        ;
    }
}

/// Updates the timer on a all sound effects. As I currently know no other way to check if a sound
/// finished playing, this is the solution.
/// It doesn't matter if the sound plays longer than its timer, the entity can be deleted
/// anyway without interrupting it.
fn update_sound_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut sounds: Query<(Entity, &mut SoundEffect)>
) {
    let delta = time.delta();

    for (entity, mut sound) in &mut sounds {
        sound.update(delta);

        if sound.finished() {
            commands.entity(entity).despawn();
        }
    }
}