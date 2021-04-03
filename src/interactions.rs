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

fn pacman_eat_dot(commands: &mut Commands,
                  mut eaten_events: ResMut<Events<DotEaten>>,
                  pacman_positions: Query<&Position, With<Pacman>>,
                  dot_positions: Query<(Entity, &Position), With<Dot>>) {
    for pacman_pos in pacman_positions.iter() {
        for (entity, dot_pos) in dot_positions.iter() {
            if pacman_pos == dot_pos {
                commands.despawn(entity);
                eaten_events.send(DotEaten)
            }
        }
    }
}

fn pacman_eat_energizer(commands: &mut Commands,
                        mut eaten_events: ResMut<Events<EnergizerEaten>>,
                        pacman_positions: Query<&Position, With<Pacman>>,
                        energizer_positions: Query<(Entity, &Position), With<Energizer>>) {
    for pacman_position in pacman_positions.iter() {
        for (energizer_entity, energizer_position) in energizer_positions.iter() {
            if energizer_position == pacman_position {
                commands.despawn(energizer_entity);
                eaten_events.send(EnergizerEaten)
            }
        }
    }
}

fn ghost_hits_pacman(commands: &mut Commands,
                     mut pacman_killed_events: ResMut<Events<PacmanKilled>>,
                     pacman_query: Query<(Entity, &Position), With<Pacman>>,
                     ghost_query: Query<&Position, With<Ghost>>) {
    for (pacman_entity, pacman_position) in pacman_query.iter() {
        for ghost_position in ghost_query.iter() {
            if pacman_position == ghost_position {
                commands.despawn(pacman_entity);
                pacman_killed_events.send(PacmanKilled)
            }
        }
    }
}