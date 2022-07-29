use bevy::prelude::*;
use crate::pacman::{PacmanDead, PacmanHit};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Running,
    PacmanHit,
    PacmanDying,
    PacmanDead
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(GameState::Running)
            .add_system_set(
                SystemSet::on_update(GameState::Running).with_system(switch_to_dying_when_pacman_was_hit)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PacmanHit).with_system(start_hit_timer)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PacmanHit).with_system(switch_when_hit_timer_finished)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PacmanDying).with_system(switch_to_dead_when_pacman_is_dead)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PacmanDead).with_system(start_dead_timer)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PacmanDead).with_system(switch_when_dead_timer_finished)
            )
        ;
    }
}

#[derive(Deref, DerefMut)]
struct HitTimer(Timer);

#[derive(Deref, DerefMut)]
struct DeadTimer(Timer);

fn switch_to_dying_when_pacman_was_hit(
    mut event_reader: EventReader<PacmanHit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in event_reader.iter() {
        game_state.set(GameState::PacmanHit).unwrap()
    }
}

fn start_hit_timer(
    mut commands: Commands,
) {
    commands.insert_resource(HitTimer(Timer::from_seconds(1.0, false)))
}

fn switch_when_hit_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut hit_timer: ResMut<HitTimer>,
    mut game_state: ResMut<State<GameState>>
) {
    hit_timer.tick(time.delta());

    if hit_timer.finished() {
        commands.remove_resource::<HitTimer>();
        game_state.set(GameState::PacmanDying).unwrap()
    }
}

fn switch_to_dead_when_pacman_is_dead(
    mut event_reader: EventReader<PacmanDead>,
    mut game_state: ResMut<State<GameState>>
) {
    for _ in event_reader.iter() {
        game_state.set(GameState::PacmanDead).unwrap()
    }
}

fn start_dead_timer(
    mut commands: Commands
) {
    commands.insert_resource(DeadTimer(Timer::from_seconds(1.0, false)))
}

fn switch_when_dead_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut dead_timer: ResMut<DeadTimer>,
    mut game_state: ResMut<State<GameState>>
) {
    dead_timer.tick(time.delta());

    if dead_timer.finished() {
        commands.remove_resource::<DeadTimer>();
        game_state.set(GameState::Running).unwrap()
    }
}

