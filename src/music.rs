use bevy::audio::AudioSink;
use bevy::prelude::*;
use LifeCycle::Start;
use crate::game::edibles::dots::EatenDots;
use crate::game::edibles::energizer::EnergizerTimer;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::ghosts::Ghost;
use crate::life_cycle::LifeCycle;
use crate::life_cycle::LifeCycle::Running;
use crate::game::state::State;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBackground::Siren1)
            .add_system_set(
                SystemSet::on_enter(Start)
                    .with_system(play_start_sound)
                    .with_system(init_noises)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(update_current_background)
                    .with_system(set_background)
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

fn init_noises(
    mut commands: Commands,
    audio: Res<Audio>,
    loaded_assets: Res<LoadedAssets>,
    sinks: Res<Assets<AudioSink>>,
) {
    let start_audio = |asset: &'static str| audio.play_with_settings(loaded_assets.get_handle(asset), PlaybackSettings::LOOP.with_volume(0.0));

    commands.insert_resource(NoiseHandles {
        siren: sinks.get_handle((start_audio)("sounds/siren.ogg")),
        frightened: sinks.get_handle((start_audio)("sounds/frightened.ogg")),
        eaten: sinks.get_handle((start_audio)("sounds/eaten.ogg")),
    });
}

fn mute(
    sinks: Res<Assets<AudioSink>>,
    noise_handles: Res<NoiseHandles>
) {
    noise_handles.mute_all(&sinks);
}

fn update_current_background(
    mut current_background: ResMut<CurrentBackground>,
    energizer_timer_opt: Option<Res<EnergizerTimer>>,
    eaten_dots: Res<EatenDots>,
    query: Query<&State, With<Ghost>>
) {
    let eaten_ghosts = query.iter().filter(|s| s == &&State::Eaten).count();

    if eaten_ghosts > 0 {
        *current_background = CurrentBackground::Eaten
    } else if let Some(_) = energizer_timer_opt {
        *current_background = CurrentBackground::Frightened
    } else {
        *current_background = match eaten_dots.get_eaten() as f32 / eaten_dots.get_max() as f32 {
            r if r >= 0.0 && r < 0.25 => CurrentBackground::Siren1,
            r if r >= 0.25 && r < 0.5 => CurrentBackground::Siren2,
            r if r >= 0.5 && r < 0.75 => CurrentBackground::Siren3,
            _ => CurrentBackground::Siren4,
        }
    }
}

fn set_background(
    current_background: Res<CurrentBackground>,
    sinks: Res<Assets<AudioSink>>,
    noise_handles: Res<NoiseHandles>
) {
    if !current_background.is_changed() {
        return;
    }

    match *current_background {
        CurrentBackground::Siren1 => noise_handles.play_siren(&sinks, 1.0),
        CurrentBackground::Siren2 => noise_handles.play_siren(&sinks, 1.05),
        CurrentBackground::Siren3 => noise_handles.play_siren(&sinks, 1.1),
        CurrentBackground::Siren4 => noise_handles.play_siren(&sinks, 1.15),
        CurrentBackground::Frightened => noise_handles.play_frightened(&sinks),
        CurrentBackground::Eaten => noise_handles.play_eaten(&sinks),
    }
}

#[derive(Resource)]
struct NoiseHandles {
    siren: Handle<AudioSink>,
    frightened: Handle<AudioSink>,
    eaten: Handle<AudioSink>
}

impl NoiseHandles {
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
enum CurrentBackground {
    Siren1,
    Siren2,
    Siren3,
    Siren4,
    Frightened,
    Eaten,
}