use std::time::Duration;
use bevy::prelude::*;

use crate::constants::ENERGIZER_DIMENSION;
use crate::pacman::Pacman;
use crate::common::position::ToPosition;
use crate::is;
use crate::level::Level;
use crate::map::board::Board;
use crate::map::Element::EnergizerSpawn;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnergizerEaten>()
            .add_event::<EnergizerOver>()
            .insert_resource(EnergizerTimer::new())
            .add_startup_system(spawn_energizer)
            .add_system(pacman_eat_energizer)
            .add_system(start_energizer_timer_when_energizer_eaten.after(pacman_eat_energizer))
            .add_system(update_energizer_timer.after(start_energizer_timer_when_energizer_eaten))
        ;
    }
}

/// An energizer that allows pacman to eat ghosts.
#[derive(Component)]
pub struct Energizer;

/// Fired when pacman eats an energizer.
pub struct EnergizerEaten;

/// Fired when an energizer is no longer active
pub struct EnergizerOver;

pub struct EnergizerTimer {
    timer: Option<Timer>,
}

impl EnergizerTimer {
    pub fn new() -> Self {
        EnergizerTimer {
            timer: None
        }
    }

    /// The energizer is active for the full time at level 1.
    /// Its time gets reduced every level until level 19, were it stops instantly.
    ///
    /// I use a linear function to calculate the energizer time per level. This is only speculation.
    /// It is unclear how the time an energizer is active gets calculated.
    pub fn start(&mut self, level: &Level) {
        let level = **level as f32 - 1.0;
        let time = f32::max(8.0 - level * (8.0 / 18.0), 0.0);
        self.timer = Some(Timer::from_seconds(time, false))
    }

    pub fn tick(&mut self, delta: Duration) {
        if let Some(ref mut t) = self.timer {
            t.tick(delta);
        }

        if self.is_finished() {
            self.timer = None
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.timer {
            Some(ref t) => t.finished(),
            None => true
        }
    }
}

fn spawn_energizer(
    mut commands: Commands,
    board: Res<Board>
) {
    let energizer_dimension = Vec2::new(ENERGIZER_DIMENSION, ENERGIZER_DIMENSION);
    for position in board.get_positions_matching(is!(EnergizerSpawn)) {
        commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.0, 0.9),
                    custom_size: Some(energizer_dimension),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::from(position)),
                ..Default::default()
            })
            .insert(Energizer);
    }
}

fn pacman_eat_energizer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerEaten>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    energizer_positions: Query<(Entity, &Transform), With<Energizer>>,
) {
    for pacman_transform in pacman_positions.iter() {
        for (energizer_entity, energizer_transform) in energizer_positions.iter() {
            if energizer_transform.pos() == pacman_transform.pos() {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EnergizerEaten)
            }
        }
    }
}

fn start_energizer_timer_when_energizer_eaten(
    mut event_reader: EventReader<EnergizerEaten>,
    level: Res<Level>,
    mut energizer_timer: ResMut<EnergizerTimer>
) {
    for _ in event_reader.iter() {
        energizer_timer.start(&level)
    }
}

fn update_energizer_timer(
    mut event_writer: EventWriter<EnergizerOver>,
    mut energizer_timer: ResMut<EnergizerTimer>,
    time: Res<Time>
) {
    energizer_timer.tick(time.delta());

    if energizer_timer.is_finished() {
        event_writer.send(EnergizerOver)
    }
}