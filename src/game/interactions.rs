use bevy::prelude::*;
use crate::game::position::Position;

use crate::game::edibles::dots::{Dot, EatenDots};
use crate::game::edibles::energizer::Energizer;
use crate::game::edibles::fruit::{Fruit, FruitDespawnTimer};
use crate::game::ghosts::{CurrentlyEatenGhost, Ghost};
use crate::game::state::State;
use crate::game_state::GameState::Running;
use crate::game::pacman::Pacman;

pub(in crate::game) struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EPacmanHit>()
            .add_event::<EGhostEaten>()
            .add_event::<EDotEaten>()
            .add_event::<EEnergizerEaten>()
            .add_event::<EFruitEaten>()
            // TODO is this double set setup still valid?
            .add_systems(Update, (
                pacman_hits_ghost.in_set(LPacmanGhostHitDetection),
                pacman_eat_dot,
                pacman_eat_energizer,
                eat_fruit_when_pacman_touches_it
            ).in_set(LPacmanEnergizerHitDetection).run_if(in_state(Running)))
        ;
    }
}

/// Marks systems that check hits between pacman and ghosts
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct LPacmanGhostHitDetection;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct LPacmanEnergizerHitDetection;

/// Fired when pacman was hit by a ghost.
#[derive(Event)]
pub struct EPacmanHit;

/// Fired when Pacman ate a ghost in frightened state.
/// Contains the eaten ghost entity and transform.
#[derive(Event, Copy, Clone)]
pub struct EGhostEaten(pub Entity, pub Transform);

/// Fired when pacman eats a dot.
#[derive(Event)]
pub struct EDotEaten;

/// Fired when pacman eats an energizer.
#[derive(Event, Copy, Clone)]
pub struct EEnergizerEaten;

/// Event that gets fired when pacman ate a fruit.
#[derive(Event)]
pub struct EFruitEaten(pub Fruit, pub Transform);

fn pacman_hits_ghost(
    mut commands: Commands,
    mut killed_event_writer: EventWriter<EPacmanHit>,
    mut eat_event_writer: EventWriter<EGhostEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in &pacman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            if Position::from_vec(&pacman_transform.translation) == Position::from_vec(&ghost_transform.translation) {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(EPacmanHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(EGhostEaten(entity, *ghost_transform));
                    commands.insert_resource(CurrentlyEatenGhost(entity))
                }
            }
        }
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<EDotEaten>,
    mut eaten_dots: ResMut<EatenDots>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for pacman_tf in &pacman_positions {
        for (entity, dot_tf) in &dot_positions {
            if Position::from_vec(&pacman_tf.translation) == Position::from_vec(&dot_tf.translation) {
                commands.entity(entity).despawn();
                eaten_dots.increment();
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
            if Position::from_vec(&energizer_transform.translation) == Position::from_vec(&pacman_transform.translation) {
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
    fruit_query: Query<(Entity, &Fruit, &Transform)>,
) {
    for pacman_tf in &pacman_query {
        for (entity, fruit, fruit_tf) in &fruit_query {
            if Position::from_vec(&pacman_tf.translation) == Position::from_vec(&fruit_tf.translation) {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(EFruitEaten(*fruit, *fruit_tf))
            }
        }
    }
}