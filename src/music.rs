use bevy::audio::{AudioSink, Volume};
use bevy::prelude::*;
use crate::game_state::GameState::Start;
use crate::game::edibles::dots::EatenDots;
use crate::game::edibles::energizer::EnergizerTimer;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::ghosts::Ghost;
use crate::game_state::GameState::Running;
use crate::game::state::State;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentTrack::Siren1)
            .add_systems(OnEnter(Start), (
                play_start_sound,
                init_background_music
            ))
            .add_systems(Update, (
                update_current_track,
                play_track
            ).run_if(in_state(Running)))
            .add_systems(OnExit(Running), mute)
        ;
    }
}

/// Marker for a background track
#[derive(Component)]
struct BackgroundTrack;

/// Marker for the siren background track
#[derive(Component)]
struct SirenBackground;

/// Marker for the frightened background track
#[derive(Component)]
struct FrightenedBackground;

/// Marker for the frightened eaten track
#[derive(Component)]
struct EatenBackground;

fn play_start_sound(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    commands.spawn(
        AudioBundle {
            source: loaded_assets.get_handle("sounds/start.ogg"),
            ..default()
        }
    );
}

/// Starts every background track at the same time with volume of 0.
fn init_background_music(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    start_background_track(&mut commands, &loaded_assets, SirenBackground, "sounds/siren.ogg");
    start_background_track(&mut commands, &loaded_assets, FrightenedBackground, "sounds/frightened.ogg");
    start_background_track(&mut commands, &loaded_assets, EatenBackground, "sounds/eaten.ogg");
}

fn start_background_track(
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
    marker: impl Component,
    name: &'static str,
) {
    commands.spawn((
        marker,
        AudioBundle {
            source: loaded_assets.get_handle(name),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.0)),
            ..default()
        }
    ));
}

fn mute(
    background_tracks: Query<&AudioSink, With<BackgroundTrack>>,
) {
    background_tracks.for_each(|sink| sink.set_volume(0.0));
}

fn update_current_track(
    mut current_track: ResMut<CurrentTrack>,
    energizer_timer_opt: Option<Res<EnergizerTimer>>,
    eaten_dots: Res<EatenDots>,
    query: Query<&State, With<Ghost>>,
) {
    let eaten_ghosts = query.iter().filter(|s| s == &&State::Eaten).count();

    if eaten_ghosts > 0 {
        *current_track = CurrentTrack::Eaten
    } else if let Some(_) = energizer_timer_opt {
        *current_track = CurrentTrack::Frightened
    } else {
        *current_track = match eaten_dots.get_eaten() as f32 / eaten_dots.get_max() as f32 {
            r if r >= 0.0 && r < 0.25 => CurrentTrack::Siren1,
            r if r >= 0.25 && r < 0.5 => CurrentTrack::Siren2,
            r if r >= 0.5 && r < 0.75 => CurrentTrack::Siren3,
            _ => CurrentTrack::Siren4,
        }
    }
}

fn play_track(
    current_track: Res<CurrentTrack>,
    siren_tracks: Query<&AudioSink, With<SirenBackground>>,
    frightened_tracks: Query<&AudioSink, With<FrightenedBackground>>,
    eaten_tracks: Query<&AudioSink, With<EatenBackground>>,
) {
    if !current_track.is_changed() {
        return;
    }

    let mixer = match (
        siren_tracks.get_single(),
        frightened_tracks.get_single(),
        eaten_tracks.get_single()
    ) {
        (Ok(siren), Ok(frightened), Ok(eaten)) => Mixer::new(siren, frightened, eaten),
        _ => return
    };

    match *current_track {
        CurrentTrack::Siren1 => mixer.play_siren_1(),
        CurrentTrack::Siren2 => mixer.play_siren_2(),
        CurrentTrack::Siren3 => mixer.play_siren_3(),
        CurrentTrack::Siren4 => mixer.play_siren_4(),
        CurrentTrack::Frightened => mixer.play_frightened(),
        CurrentTrack::Eaten => mixer.play_eaten(),
    }
}

struct Mixer<'a> {
    siren_track: &'a AudioSink,
    frightened_track: &'a AudioSink,
    eaten_track: &'a AudioSink,
}

impl<'a> Mixer<'a> {
    pub fn new(siren_track: &'a AudioSink, frightened_track: &'a AudioSink, eaten_track: &'a AudioSink) -> Self {
        Self { siren_track, frightened_track, eaten_track }
    }

    fn play_siren_1(self) {
        self.play_siren(1.0)
    }

    fn play_siren_2(self) {
        self.play_siren(1.05)
    }

    fn play_siren_3(self) {
        self.play_siren(1.1)
    }

    fn play_siren_4(self) {
        self.play_siren(1.15)
    }

    fn play_siren(self, speed: f32) {
        self.siren_track.set_volume(1.0);
        self.siren_track.set_speed(speed);
        self.frightened_track.set_volume(0.0);
        self.eaten_track.set_volume(0.0);
    }

    fn play_frightened(self) {
        self.siren_track.set_volume(0.0);
        self.frightened_track.set_volume(1.0);
        self.eaten_track.set_volume(0.0);
    }

    fn play_eaten(self) {
        self.siren_track.set_volume(0.0);
        self.frightened_track.set_volume(0.0);
        self.eaten_track.set_volume(1.0);
    }
}

#[derive(Resource)]
enum CurrentTrack {
    Siren1,
    Siren2,
    Siren3,
    Siren4,
    Frightened,
    Eaten,
}