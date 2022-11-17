use std::time::Duration;
use bevy::prelude::*;

use crate::constants::ENERGIZER_DIMENSION;
use crate::game::edibles::Edible;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::interactions::EEnergizerEaten;
use crate::game_state::GameState::*;
use crate::game::level::Level;
use crate::game::map::EnergizerSpawn;
use crate::game::specs_per_level::SpecsPerLevel;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnergizerOver>()
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_energizer)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(start_energizer_timer_when_energizer_eaten)
                    .with_system(update_energizer_timer.after(start_energizer_timer_when_energizer_eaten))
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition).with_system(spawn_energizer)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanHit).with_system(despawn_energizer_timer)
            )
            .add_system_set(
                SystemSet::on_enter(LevelTransition).with_system(despawn_energizer_timer)
            )
        ;
    }
}

/// Parent component for all energizer (for organization only)
#[derive(Component)]
pub struct Energizers;

/// An energizer that allows pacman to eat ghosts.
#[derive(Component)]
pub struct Energizer;

/// Fired when an energizer is no longer active
#[derive(Copy, Clone)]
pub struct EnergizerOver;

#[derive(Resource)]
pub struct EnergizerTimer {
    timer: Timer,
}

impl EnergizerTimer {
    fn start(seconds: f32) -> Self {
        EnergizerTimer {
            timer: Timer::from_seconds(seconds, TimerMode::Once)
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }

    /// Return the remaining seconds for this timer (if the timer is active, else None)
    pub fn remaining(&self) -> f32 {
        self.timer.duration().as_secs_f32() - self.timer.elapsed_secs()
    }
}

fn spawn_energizer(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    spawn_query: Query<&EnergizerSpawn>,
) {
    let energizers = commands.spawn((
        Name::new("Energizers"),
        Energizers,
        SpatialBundle::default()
    )).id();

    for spawn in &spawn_query {
        commands.entity(energizers).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: loaded_assets.get_handle("textures/energizer.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(ENERGIZER_DIMENSION)),
                        ..default()
                    },
                    transform: Transform::from_translation(**spawn),
                    ..Default::default()
                },
                Energizer,
                Edible,
                Name::new("Energizer")
            ));
        });
    }
}

fn start_energizer_timer_when_energizer_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EEnergizerEaten>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    for _ in event_reader.iter() {
        let spec = specs_per_level.get_for(&level);
        commands.insert_resource(EnergizerTimer::start(spec.frightened_time));
    }
}

fn update_energizer_timer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerOver>,
    energizer_timer: Option<ResMut<EnergizerTimer>>,
    time: Res<Time>,
) {
    if let Some(mut timer) = energizer_timer {
        timer.tick(time.delta());

        if timer.is_finished() {
            commands.remove_resource::<EnergizerTimer>();
            event_writer.send(EnergizerOver);
        }
    }
}

fn despawn_energizer_timer(
    mut commands: Commands,
) {
    commands.remove_resource::<EnergizerTimer>();
}