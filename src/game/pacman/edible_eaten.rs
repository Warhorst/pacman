use std::time::Duration;
use bevy::prelude::*;
use crate::prelude::*;

/// When eating dots/energizers, pacman stops for 1 or 3 Frames in the original game.
/// The systems in this plugin do the same thing, but with timers for 1/60 and 3/60 seconds
pub(crate) struct EdibleEatenPlugin;

impl Plugin for EdibleEatenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    add_edible_stop_when_dot_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    add_edible_stop_when_energizer_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    remove_edible_stop_when_timer_ended
                )
                    .run_if(in_state(Game(Running))))
        ;
    }
}

fn add_edible_stop_when_dot_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<DotWasEaten>,
    query: Query<Entity, With<Pacman>>,
) {
    for _ in event_reader.read() {
        for e in &query {
            commands.entity(e).insert(EdibleEatenStop(Timer::new(Duration::from_secs_f32(1.0 / 60.0), TimerMode::Once)));
        }
    }
}

fn add_edible_stop_when_energizer_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EnergizerWasEaten>,
    query: Query<Entity, With<Pacman>>,
) {
    for _ in event_reader.read() {
        for e in &query {
            commands.entity(e).insert(EdibleEatenStop(Timer::new(Duration::from_secs_f32(3.0 / 60.0), TimerMode::Once)));
        }
    }
}

fn remove_edible_stop_when_timer_ended(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EdibleEatenStop), With<Pacman>>,
) {
    for (e, mut stop) in &mut query {
        stop.tick(time.delta());

        if stop.finished() {
            commands.entity(e).remove::<EdibleEatenStop>();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct EdibleEatenStop(Timer);

