use bevy::prelude::*;
use crate::game::position::Position;

use crate::game::edibles::dots::{Dot, EatenDots};
use crate::game::edibles::energizer::Energizer;
use crate::game::edibles::fruit::{Fruit, FruitDespawnTimer};
use crate::game::ghosts::{CurrentlyEatenGhost, Ghost};
use crate::game::state::State;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::pacman::Pacman;
use crate::system_sets::DetectIntersectionsWithPacman;

pub(in crate::game) struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PacmanWasHit>()
            .add_event::<GhostWasEaten>()
            .add_event::<DotWasEaten>()
            .add_event::<EnergizerWasEaten>()
            .add_event::<FruitWasEaten>()
            .add_systems(
                Update,
                (
                    pacman_hits_ghost,
                    pacman_eat_dot,
                    pacman_eat_energizer,
                    eat_fruit_when_pacman_touches_it
                )
                    .in_set(DetectIntersectionsWithPacman)
                    .run_if(in_state(Game(Running))))
        ;
    }
}

/// Fired when pacman was hit by a ghost.
#[derive(Event)]
pub struct PacmanWasHit;

/// Fired when Pacman ate a ghost in frightened state.
/// Contains the eaten ghost entity and the transform to show a score on the ghosts
/// former position.
#[derive(Event, Copy, Clone)]
pub struct GhostWasEaten(pub Entity, pub Transform);

/// Fired when pacman eats a dot.
#[derive(Event)]
pub struct DotWasEaten;

/// Fired when pacman eats an energizer.
#[derive(Event, Copy, Clone)]
pub struct EnergizerWasEaten;

/// Event that gets fired when pacman ate a fruit.
/// Holds the type of fruit and the transform to show a score on the fruits
/// former position.
#[derive(Event)]
pub struct FruitWasEaten(pub Fruit, pub Transform);

fn pacman_hits_ghost(
    mut commands: Commands,
    mut killed_event_writer: EventWriter<PacmanWasHit>,
    mut eat_event_writer: EventWriter<GhostWasEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in &pacman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            if Position::from_vec(&pacman_transform.translation) == Position::from_vec(&ghost_transform.translation) {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(PacmanWasHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(GhostWasEaten(entity, *ghost_transform));
                    commands.insert_resource(CurrentlyEatenGhost(entity))
                }
            }
        }
    }
}

fn pacman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<DotWasEaten>,
    mut eaten_dots: ResMut<EatenDots>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for pacman_tf in &pacman_positions {
        for (entity, dot_tf) in &dot_positions {
            if Position::from_vec(&pacman_tf.translation) == Position::from_vec(&dot_tf.translation) {
                commands.entity(entity).despawn();
                eaten_dots.increment();
                event_writer.send(DotWasEaten)
            }
        }
    }
}

fn pacman_eat_energizer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerWasEaten>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    energizer_positions: Query<(Entity, &Transform), With<Energizer>>,
) {
    for pacman_transform in &pacman_positions {
        for (energizer_entity, energizer_transform) in &energizer_positions {
            if Position::from_vec(&energizer_transform.translation) == Position::from_vec(&pacman_transform.translation) {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EnergizerWasEaten)
            }
        }
    }
}

/// If pacman touches the fruit, despawn it, remove the timer and send an event.
fn eat_fruit_when_pacman_touches_it(
    mut commands: Commands,
    mut event_writer: EventWriter<FruitWasEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    fruit_query: Query<(Entity, &Fruit, &Transform)>,
) {
    for pacman_tf in &pacman_query {
        for (entity, fruit, fruit_tf) in &fruit_query {
            if Position::from_vec(&pacman_tf.translation) == Position::from_vec(&fruit_tf.translation) {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(FruitWasEaten(*fruit, *fruit_tf))
            }
        }
    }
}