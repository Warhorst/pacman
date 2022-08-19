use std::time::Duration;
use bevy::prelude::*;
use crate::animation::Animations;
use crate::ghosts::Ghost;
use crate::interactions::EPacmanEatsGhost;
use crate::life_cycle::LifeCycle::Running;
use crate::ghosts::state::State;

/// When a ghost was eaten, all ghosts stop for a while. Only eaten ghosts can move in this period, unless they are eaten themself.
pub(in crate::ghosts) struct GhostEatenPlugin;

impl Plugin for GhostEatenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(add_stop_when_ghost_eaten)
                    .with_system(remove_self_stop_when_timer_ended)
                    .with_system(remove_other_stop_when_timer_ended)
            )
        ;
    }
}

fn add_stop_when_ghost_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EPacmanEatsGhost>,
    mut query: Query<(Entity, &State, &mut Animations, &mut Visibility), With<Ghost>>,
) {
    for event in event_reader.iter() {
        for (e, state, mut animations, mut visibility) in &mut query {
            if e == event.0 {
                commands.entity(e).insert(SelfEatenStop(Timer::new(Duration::from_secs_f32(1.0), false)));
                visibility.is_visible = false;
            } else {
                commands.entity(e).insert(OtherEatenStop(Timer::new(Duration::from_secs_f32(1.0), false)));

                if state != &State::Eaten {
                    animations.stop();
                }
            }
        }
    }
}

fn remove_self_stop_when_timer_ended(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Visibility, &mut SelfEatenStop), With<Ghost>>,
) {
    for (e, mut visibility, mut stop) in &mut query {
        stop.tick(time.delta());

        if stop.finished() {
            commands.entity(e).remove::<SelfEatenStop>();
            visibility.is_visible = true;
        }
    }
}

fn remove_other_stop_when_timer_ended(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Animations, &mut OtherEatenStop), With<Ghost>>,
) {
    for (e, mut animations, mut stop) in &mut query {
        stop.tick(time.delta());

        if stop.finished() {
            commands.entity(e).remove::<OtherEatenStop>();
            animations.resume();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(in crate::ghosts) struct SelfEatenStop(Timer);

#[derive(Component, Deref, DerefMut)]
pub(in crate::ghosts) struct OtherEatenStop(Timer);