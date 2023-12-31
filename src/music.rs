use bevy::audio::{AudioSink, Volume};
use bevy::prelude::*;
use crate::core::prelude::*;
use crate::music::CurrentTrack::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BackgroundMusic::new_muted())
            .add_systems(
                OnEnter(Game(Start)), (
                    play_start_sound,
                    init_background_music
                ))
            .add_systems(
                OnEnter(Game(Running)),
                unmute_background_music,
            )
            .add_systems(Update, (
                update_background_music,
                play_track
            ).run_if(in_game))
            .add_systems(
                OnExit(Game(Running)),
                mute_background_music,
            )
        ;
    }
}

fn play_start_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("StartSound"),
        SoundEffect::new(),
        AudioBundle {
            source: asset_server.load("sounds/start.ogg"),
            ..default()
        }
    ));
}

/// Starts every background track at the same time with volume of 0.
fn init_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    start_background_track(&mut commands, &asset_server, SirenBackground, "sounds/siren.ogg");
    start_background_track(&mut commands, &asset_server, FrightenedBackground, "sounds/frightened.ogg");
    start_background_track(&mut commands, &asset_server, EatenBackground, "sounds/eaten.ogg");
}

fn start_background_track(
    commands: &mut Commands,
    loaded_assets: &AssetServer,
    marker: impl Component,
    path: &'static str,
) {
    commands.spawn((
        Name::new(path),
        marker,
        AudioBundle {
            source: loaded_assets.load(path),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.0)),
        }
    ));
}

/// Unmute the background music.
fn unmute_background_music(
    mut background_music: ResMut<BackgroundMusic>,
) {
    background_music.muted = false
}

/// Mute the background music.
fn mute_background_music(
    mut background_music: ResMut<BackgroundMusic>,
) {
    background_music.muted = true
}

fn update_background_music(
    mut background_music: ResMut<BackgroundMusic>,
    energizer_timer_opt: Option<Res<EnergizerTimer>>,
    eaten_dots: Res<EatenDots>,
    query: Query<&GhostState, With<Ghost>>,
) {
    let eaten_ghosts = query.iter().filter(|s| s == &&GhostState::Eaten).count();

    if eaten_ghosts > 0 {
        background_music.current_track = EatenTrack
    } else if energizer_timer_opt.is_some() {
        background_music.current_track = FrightenedTrack
    } else {
        background_music.current_track = match eaten_dots.get_eaten() as f32 / eaten_dots.get_max() as f32 {
            r if (0.0..0.25).contains(&r) => Siren1,
            r if (0.25..0.5).contains(&r) => Siren2,
            r if (0.5..0.75).contains(&r) => Siren3,
            _ => Siren4,
        }
    }
}

fn play_track(
    background_music: Res<BackgroundMusic>,
    siren_tracks: Query<&AudioSink, With<SirenBackground>>,
    frightened_tracks: Query<&AudioSink, With<FrightenedBackground>>,
    eaten_tracks: Query<&AudioSink, With<EatenBackground>>,
) {
    if !background_music.is_changed() {
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

    if background_music.muted {
        mixer.mute_all();
        return;
    }

    match &background_music.current_track {
        Siren1 => mixer.play_siren_1(),
        Siren2 => mixer.play_siren_2(),
        Siren3 => mixer.play_siren_3(),
        Siren4 => mixer.play_siren_4(),
        FrightenedTrack => mixer.play_frightened(),
        EatenTrack => mixer.play_eaten(),
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

    fn mute_all(self) {
        self.siren_track.set_volume(0.0);
        self.frightened_track.set_volume(0.0);
        self.eaten_track.set_volume(0.0);
    }
}