use std::time::Duration;
use bevy::prelude::*;
use crate::interactions::EPacmanEatsGhost;
use crate::life_cycle::LifeCycle::Running;
use crate::pacman::Pacman;

/// When eating a ghost, pacman stops and gets invisible for a short time.
pub (in crate::pacman) struct GhostEatenPlugin;

impl Plugin for GhostEatenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(add_ghost_stop_when_ghost_eaten)
                    .with_system(remove_ghost_stop_when_timer_ended)
            )
        ;
    }
}

fn add_ghost_stop_when_ghost_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EPacmanEatsGhost>,
    mut query: Query<(Entity, &mut Visibility), With<Pacman>>
) {
    for _ in event_reader.iter() {
        for (e, mut visibility) in &mut query {
            visibility.is_visible = false;
            commands.entity(e).insert(GhostEatenStop(Timer::new(Duration::from_secs_f32(1.0), false)));
        }
    }
}

fn remove_ghost_stop_when_timer_ended(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Visibility, &mut GhostEatenStop), With<Pacman>>
) {
    for (e, mut visibility, mut stop) in &mut query {
        stop.tick(time.delta());

        if stop.finished() {
            visibility.is_visible = true;
            commands.entity(e).remove::<GhostEatenStop>();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub (in crate::pacman) struct GhostEatenStop(Timer);