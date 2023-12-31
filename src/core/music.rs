use bevy::prelude::*;
use crate::core::prelude::*;

pub(super) struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<BackgroundTrack>()
            .register_type::<SirenBackground>()
            .register_type::<FrightenedBackground>()
            .register_type::<BackgroundMusic>()
        ;
    }
}

/// Marker for a background track
#[derive(Component, Reflect)]
pub struct BackgroundTrack;

/// Marker for the siren background track
#[derive(Component, Reflect)]
pub struct SirenBackground;

/// Marker for the frightened background track
#[derive(Component, Reflect)]
pub struct FrightenedBackground;

/// Marker for the frightened eaten track
#[derive(Component, Reflect)]
pub struct EatenBackground;

/// Controller for the music that will play in the background
#[derive(Resource, Reflect)]
pub struct BackgroundMusic {
    pub current_track: CurrentTrack,
    pub muted: bool,
}

impl BackgroundMusic {
    pub fn new_muted() -> Self {
        BackgroundMusic {
            current_track: Siren1,
            muted: true,
        }
    }
}

/// Identifiers for the current track that should be played
#[derive(Reflect)]
pub enum CurrentTrack {
    /// The siren sound, when the remaining dots are between 100% and 75%
    Siren1,
    /// The siren sound, when the remaining dots are between 75% and 50%
    Siren2,
    /// The siren sound, when the remaining dots are between 50% and 25%
    Siren3,
    /// The siren sound, when the remaining dots are between 25% and 0%
    Siren4,
    /// The music that plays when pacman ate an energizer and the ghosts turn blue
    FrightenedTrack,
    /// The sound that plays when an eaten ghost returns to the ghost house
    EatenTrack,
}