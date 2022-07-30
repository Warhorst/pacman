use bevy::prelude::*;
use crate::pacman::{EPacmanDead, EPacmanHit};
use LifeCycle::*;
use crate::lives::Life;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LifeCycle {
    Start,
    Ready,
    Running,
    PacmanHit,
    PacmanDying,
    PacmanDead,
    GameOver,
    LevelTransition
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(Start)
            .add_system_set(
                SystemSet::on_enter(Start).with_system(start_state_timer)
            )
            .add_system_set(
                SystemSet::on_update(Start).with_system(switch_state_when_state_timer_finished)
            )
            .add_system_set(
                SystemSet::on_enter(Ready).with_system(start_state_timer)
            )
            .add_system_set(
                SystemSet::on_update(Ready).with_system(switch_state_when_state_timer_finished)
            )
            .add_system_set(
                SystemSet::on_update(Running).with_system(switch_to_dying_when_pacman_was_hit)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanHit).with_system(start_state_timer)
            )
            .add_system_set(
                SystemSet::on_update(PacmanHit).with_system(switch_state_when_state_timer_finished)
            )
            .add_system_set(
                SystemSet::on_update(PacmanDying).with_system(switch_to_dead_when_pacman_is_dead)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanDead).with_system(start_state_timer)
            )
            .add_system_set(
                SystemSet::on_update(PacmanDead).with_system(switch_dead_state_when_timer_finished)
            )
        ;
    }
}

/// Some lifecycle states just wait for a few seconds before switching. This timer and the related systems
/// handle these states
#[derive(Deref, DerefMut)]
struct StateTimer(Timer);

fn start_state_timer(
    mut commands: Commands,
    life_cycle: Res<State<LifeCycle>>
) {
    let state_time = match life_cycle.current() {
        Start => 1.0,
        Ready => 1.0,
        PacmanHit => 1.0,
        PacmanDead => 1.0,
        _ => return
    };

    commands.insert_resource(StateTimer(Timer::from_seconds(state_time, false)));
}

fn switch_state_when_state_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    mut life_cycle: ResMut<State<LifeCycle>>
) {
    state_timer.tick(time.delta());

    if state_timer.finished() {
        commands.remove_resource::<StateTimer>();

        let next_state = match life_cycle.current() {
            Start => Ready,
            Ready => Running,
            PacmanHit => PacmanDying,
            _ => return
        };

        life_cycle.set(next_state).unwrap()
    }
}

fn switch_to_dying_when_pacman_was_hit(
    mut event_reader: EventReader<EPacmanHit>,
    mut game_state: ResMut<State<LifeCycle>>,
) {
    for _ in event_reader.iter() {
        game_state.set(PacmanHit).unwrap()
    }
}

fn switch_to_dead_when_pacman_is_dead(
    mut event_reader: EventReader<EPacmanDead>,
    mut game_state: ResMut<State<LifeCycle>>
) {
    for _ in event_reader.iter() {
        game_state.set(PacmanDead).unwrap()
    }
}

fn switch_dead_state_when_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    mut life_cycle: ResMut<State<LifeCycle>>,
    query: Query<&Life>
) {
    state_timer.tick(time.delta());

    if state_timer.finished() {
        commands.remove_resource::<StateTimer>();

        if query.iter().count() > 0 {
            life_cycle.set(Ready).unwrap()
        } else {
            life_cycle.set(GameOver).unwrap()
        }
    }
}

