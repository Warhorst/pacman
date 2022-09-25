use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;

use crate::edibles::dots::{Dot, EatenDots};
use crate::edibles::energizer::Energizer;
use crate::edibles::fruit::{Fruit, FruitDespawnTimer};
use crate::ghosts::{CurrentlyEatenGhost, Ghost};
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
                    .with_system(pacman_hits_ghost.label(LPacmanGhostHitDetection))
                    .with_system(pacman_eat_dot)
                    .with_system(pacman_eat_energizer)
                    .with_system(eat_fruit_when_pacman_touches_it)
            )
        ;
    }
}

/// Marks systems that check hits between pacman and ghosts
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub struct LPacmanGhostHitDetection;

/// Fired when pacman was hit by a ghost.
pub struct EPacmanHit;

/// Fired when Pacman ate a ghost in frightened state.
/// Contains the eaten ghost entity and transform.
pub struct EPacmanEatsGhost(pub Entity, pub Transform);

/// Fired when pacman eats a dot.
pub struct EDotEaten;

/// Fired when pacman eats an energizer.
pub struct EEnergizerEaten;

/// Event that gets fired when pacman ate a fruit.
pub struct EFruitEaten(pub Fruit, pub Transform);

fn pacman_hits_ghost(
    mut commands: Commands,
    mut killed_event_writer: EventWriter<EPacmanHit>,
    mut eat_event_writer: EventWriter<EPacmanEatsGhost>,
    dimensions: Res<BoardDimensions>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for pacman_transform in &pacman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            if dimensions.trans_to_pos(pacman_transform) == dimensions.trans_to_pos(ghost_transform) {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(EPacmanHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(EPacmanEatsGhost(entity, *ghost_transform));
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
    dimensions: Res<BoardDimensions>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for pacman_tf in &pacman_positions {
        for (entity, dot_tf) in &dot_positions {
            if dimensions.trans_to_pos(pacman_tf) == dimensions.trans_to_pos(dot_tf) {
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
    dimensions: Res<BoardDimensions>,
    pacman_positions: Query<&Transform, With<Pacman>>,
    energizer_positions: Query<(Entity, &Transform), With<Energizer>>,
) {
    for pacman_transform in &pacman_positions {
        for (energizer_entity, energizer_transform) in &energizer_positions {
            if dimensions.trans_to_pos(energizer_transform) == dimensions.trans_to_pos(pacman_transform) {
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
    dimensions: Res<BoardDimensions>,
    pacman_query: Query<&Transform, With<Pacman>>,
    fruit_query: Query<(Entity, &Fruit, &Transform)>
) {
    for pacman_tf in &pacman_query {
        for (entity, fruit, fruit_tf) in &fruit_query {
            if dimensions.trans_to_pos(pacman_tf) == dimensions.trans_to_pos(fruit_tf) {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(EFruitEaten(*fruit, *fruit_tf))
            }
        }
    }
}