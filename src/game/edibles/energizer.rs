use std::time::Duration;
use bevy::prelude::*;

use crate::constants::ENERGIZER_DIMENSION;
use crate::game::edibles::Edible;
use crate::game::interactions::EnergizerWasEaten;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::level::Level;
use crate::game::map::EnergizerSpawn;
use crate::game::specs_per_level::SpecsPerLevel;
use crate::game_state::in_game;
use crate::system_sets::ProcessIntersectionsWithPacman;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnergizerOver>()
            .add_systems(
                OnEnter(Game(Start)),
                spawn_energizer,
            )
            .add_systems(
                Update,
                (
                    start_energizer_timer_when_energizer_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    update_energizer_timer
                        .after(start_energizer_timer_when_energizer_eaten)
                )
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                OnExit(Game(LevelTransition)),
                spawn_energizer,
            )
            .add_systems(
                OnEnter(Game(PacmanHit)),
                despawn_energizer_timer,
            )
            .add_systems(
                OnEnter(Game(LevelTransition)),
                despawn_energizer_timer,
            )
            .add_systems(
                OnExit(Game(GameOver)),
                (
                    despawn_energizers,
                    despawn_energizer_timer
                ),
            )
            .add_systems(
                Update,
                animate_energizers
                    .run_if(in_game),
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
#[derive(Event, Copy, Clone)]
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
    asset_server: Res<AssetServer>,
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
                    texture: asset_server.load("textures/energizer.png"),
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
    mut event_reader: EventReader<EnergizerWasEaten>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    for _ in event_reader.read() {
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

fn despawn_energizers(
    mut commands: Commands,
    query: Query<Entity, With<Energizers>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Deref, DerefMut)]
struct EnergizerAnimationTimer(Timer);

impl Default for EnergizerAnimationTimer {
    fn default() -> Self {
        EnergizerAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
    }
}

/// Let energizers blink like in the real game
fn animate_energizers(
    time: Res<Time>,
    mut timer: Local<EnergizerAnimationTimer>,
    mut query: Query<&mut Visibility, With<Energizers>>,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        for mut vis in &mut query {
            *vis = match *vis {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                _ => *vis
            }
        }
    }
}