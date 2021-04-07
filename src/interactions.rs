use bevy::prelude::*;

use crate::common::Position;
use crate::dots::Dot;
use crate::energizer::Energizer;
use crate::events::{DotEaten, EnergizerEaten, PacmanKilled};
use crate::ghosts::components::Ghost;
use crate::pacman::Pacman;

pub struct InteractionsPlugin;

/// Plugin that fires events when specific interactions between entities happen.
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(pacman_eat_dot.system())
            .add_system(pacman_eat_energizer.system())
            .add_system(ghost_hits_pacman.system());
    }
}

fn pacman_eat_dot(mut commands: Commands,
                  mut event_writer: EventWriter<DotEaten>,
                  pacman_positions: Query<&Position, With<Pacman>>,
                  dot_positions: Query<(Entity, &Position), With<Dot>>) {
    for pacman_pos in pacman_positions.iter() {
        for (entity, dot_pos) in dot_positions.iter() {
            if pacman_pos == dot_pos {
                commands.entity(entity).despawn();
                event_writer.send(DotEaten)
            }
        }
    }
}

fn pacman_eat_energizer(mut commands: Commands,
                        mut event_writer: EventWriter<EnergizerEaten>,
                        pacman_positions: Query<&Position, With<Pacman>>,
                        energizer_positions: Query<(Entity, &Position), With<Energizer>>) {
    for pacman_position in pacman_positions.iter() {
        for (energizer_entity, energizer_position) in energizer_positions.iter() {
            if energizer_position == pacman_position {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EnergizerEaten)
            }
        }
    }
}

fn ghost_hits_pacman(mut commands: Commands,
                     mut event_writer: EventWriter<PacmanKilled>,
                     pacman_query: Query<(Entity, &Position), With<Pacman>>,
                     ghost_query: Query<&Position, With<Ghost>>) {
    for (pacman_entity, pacman_position) in pacman_query.iter() {
        for ghost_position in ghost_query.iter() {
            if pacman_position == ghost_position {
                commands.entity(pacman_entity).despawn();
                event_writer.send(PacmanKilled)
            }
        }
    }
}