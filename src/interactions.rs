use bevy::prelude::*;

use crate::common::position::ToPosition;
use crate::edibles::dots::Dot;
use crate::edibles::energizer::Energizer;
use crate::edibles::fruit::{Fruit, FruitDespawnTimer};
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::life_cycle::LifeCycle::Running;
use crate::pacman::Pacman;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EPacmanHit>()
            .add_event::<EPacmanEatsGhost>()
            .add_event::<EDotEaten>()
            .add_event::<EEnergizerEaten>()
            .add_event::<EFruitEaten>()
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(pacman_hits_ghost)
                    .with_system(pacman_eat_dot)
                    .with_system(pacman_eat_energizer)
                    .with_system(eat_fruit_when_pacman_touches_it)
            )
        ;
    }
}

/// Fired when pacman was hit by a ghost.
pub struct EPacmanHit;

/// Fired when Pacman ate a ghost in frightened state.
pub struct EPacmanEatsGhost(pub Entity, pub Transform);

/// Fired when pacman eats a dot.
pub struct EDotEaten;

/// Fired when pacman eats an energizer.
pub struct EEnergizerEaten;

/// Event that gets fired when pacman ate a fruit.
pub struct EFruitEaten(pub Fruit, pub Transform);

fn pacman_hits_ghost(
    mut killed_event_writer: EventWriter<EPacmanHit>,
    mut eat_event_writer: EventWriter<EPacmanEatsGhost>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in &pacman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            if pacman_transform.pos() == ghost_transform.pos() {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(EPacmanHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(EPacmanEatsGhost(entity, *ghost_transform))
                }
            }
        }
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<EDotEaten>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for pacman_tf in &pacman_positions {
        for (entity, dot_tf) in &dot_positions {
            if pacman_tf.pos() == dot_tf.pos() {
                commands.entity(entity).despawn();
                event_writer.send(EDotEaten)
            }
        }
    }
}

fn pacman_eat_energizer(
    mut commands: Commands,
    mut event_writer: EventWriter<EEnergizerEaten>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    energizer_positions: Query<(Entity, &Transform), With<Energizer>>,
) {
    for pacman_transform in &pacman_positions {
        for (energizer_entity, energizer_transform) in &energizer_positions {
            if energizer_transform.pos() == pacman_transform.pos() {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EEnergizerEaten)
            }
        }
    }
}

/// If pacman touches the fruit, despawn it, remove the timer and send an event.
fn eat_fruit_when_pacman_touches_it(
    mut commands: Commands,
    mut event_writer: EventWriter<EFruitEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    fruit_query: Query<(Entity, &Fruit, &Transform)>
) {
    for pacman_tf in &pacman_query {
        for (entity, fruit, fruit_tf) in &fruit_query {
            if pacman_tf.pos() == fruit_tf.pos() {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(EFruitEaten(*fruit, *fruit_tf))
            }
        }
    }
}