use bevy::prelude::*;
use crate::core::prelude::*;

pub(in crate::game) struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app

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

fn pacman_hits_ghost(
    mut commands: Commands,
    mut killed_event_writer: EventWriter<PacmanWasHit>,
    mut eat_event_writer: EventWriter<GhostWasEaten>,
    pacman_query: Query<&Transform, With<Pacman>>,
    ghost_query: Query<(Entity, &Transform, &GhostState), With<Ghost>>,
) {
    for pacman_transform in &pacman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            let pacman_pos = Pos::from_vec3(pacman_transform.translation);
            let ghost_pos = Pos::from_vec3(ghost_transform.translation);

            if pacman_pos == ghost_pos {
                if let Scatter | Chase = state {
                    killed_event_writer.write(PacmanWasHit);
                }

                if let Frightened = state {
                    eat_event_writer.write(GhostWasEaten(entity, *ghost_transform));
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
            let pacman_pos = Pos::from_vec3(pacman_tf.translation);
            let dot_pos = Pos::from_vec3(dot_tf.translation);

            if pacman_pos == dot_pos {
                commands.entity(entity).despawn();
                eaten_dots.increment();
                event_writer.write(DotWasEaten);
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
            let energizer_pos = Pos::from_vec3(energizer_transform.translation);
            let pacman_pos = Pos::from_vec3(pacman_transform.translation);

            if energizer_pos == pacman_pos {
                commands.entity(energizer_entity).despawn();
                event_writer.write(EnergizerWasEaten);
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
            let pacman_pos = Pos::from_vec3(pacman_tf.translation);
            let fruit_pos = Pos::from_vec3(fruit_tf.translation);

            if pacman_pos == fruit_pos {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.write(FruitWasEaten(*fruit, *fruit_tf));
            }
        }
    }
}