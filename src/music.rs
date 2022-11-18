use bevy::audio::AudioSink;
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
            .add_system_set(
                SystemSet::on_enter(Start)
                    .with_system(play_start_sound)
                    .with_system(init_music)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(update_current_track)
                    .with_system(play_track)
            )
            .add_system_set(
                SystemSet::on_exit(Running).with_system(mute)
            )
        ;
    }
}

fn play_start_sound(
    loaded_assets: Res<LoadedAssets>,
    audio: Res<Audio>
) {
    audio.play(loaded_assets.get_handle("sounds/start.ogg"));
}

fn init_music(
    mut commands: Commands,
    audio: Res<Audio>,
    loaded_assets: Res<LoadedAssets>,
    sinks: Res<Assets<AudioSink>>,
) {
    let start_audio = |asset: &'static str| audio.play_with_settings(loaded_assets.get_handle(asset), PlaybackSettings::LOOP.with_volume(0.0));

    commands.insert_resource(MusicHandles {
        siren: sinks.get_handle((start_audio)("sounds/siren.ogg")),
        frightened: sinks.get_handle((start_audio)("sounds/frightened.ogg")),
        eaten: sinks.get_handle((start_audio)("sounds/eaten.ogg")),
    });
}

fn mute(
    sinks: Res<Assets<AudioSink>>,
    music_handles: Res<MusicHandles>
) {
    music_handles.mute_all(&sinks);
}

fn update_current_track(
    mut current_track: ResMut<CurrentTrack>,
    energizer_timer_opt: Option<Res<EnergizerTimer>>,
    eaten_dots: Res<EatenDots>,
    query: Query<&State, With<Ghost>>
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
    sinks: Res<Assets<AudioSink>>,
    music_handles: Res<MusicHandles>
) {
    if !current_track.is_changed() {
        return;
    }

    match *current_track {
        CurrentTrack::Siren1 => music_handles.play_siren(&sinks, 1.0),
        CurrentTrack::Siren2 => music_handles.play_siren(&sinks, 1.05),
        CurrentTrack::Siren3 => music_handles.play_siren(&sinks, 1.1),
        CurrentTrack::Siren4 => music_handles.play_siren(&sinks, 1.15),
        CurrentTrack::Frightened => music_handles.play_frightened(&sinks),
        CurrentTrack::Eaten => music_handles.play_eaten(&sinks),
    }
}

#[derive(Resource)]
struct MusicHandles {
    siren: Handle<AudioSink>,
    frightened: Handle<AudioSink>,
    eaten: Handle<AudioSink>
}

impl MusicHandles {
    fn play_siren(&self, sinks: &Assets<AudioSink>, speed: f32) {
        if let Some(sink) = sinks.get(&self.siren) {
            sink.set_volume(1.0);
            sink.set_speed(speed)
        }

        Self::mute(&self.eaten, sinks);
        Self::mute(&self.frightened, sinks);
    }

    fn play_frightened(&self, sinks: &Assets<AudioSink>) {
        Self::play(&self.frightened, sinks);
        Self::mute(&self.siren, sinks);
        Self::mute(&self.eaten, sinks);
    }

    fn play_eaten(&self, sinks: &Assets<AudioSink>) {
        Self::play(&self.eaten, sinks);
        Self::mute(&self.siren, sinks);
        Self::mute(&self.frightened, sinks);
    }

    fn play(handle: &Handle<AudioSink>, sinks: &Assets<AudioSink>) {
        if let Some(sink) = sinks.get(handle) {
            sink.set_volume(1.0);
        }
    }

    fn mute_all(&self, sinks: &Assets<AudioSink>) {
        Self::mute(&self.siren, sinks);
        Self::mute(&self.eaten, sinks);
        Self::mute(&self.frightened, sinks);
    }

    fn mute(handle: &Handle<AudioSink>, sinks: &Assets<AudioSink>) {
        if let Some(sink) = sinks.get(handle) {
            sink.set_volume(0.0);
        }
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