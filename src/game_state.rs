use bevy::prelude::*;
use GameState::*;
use crate::game::edibles::EAllEdiblesEaten;
use crate::game::interactions::{EGhostEaten, EPacmanHit};
use crate::game::lives::Lives;
use crate::game_assets::EAllAssetsLoaded;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(Loading)
            .add_system(update_state)
            .add_system(update_state_timer)
        ;
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Loading,
    InGame,
    Start,
    Ready,
    Running,
    PacmanHit,
    PacmanDying,
    PacmanDead,
    GameOver,
    LevelTransition,
    GhostEatenPause
}

#[derive(Deref, DerefMut, Resource)]
struct StateTimer(Timer);

/// Update the current game state based on multiple factors.
///
/// The idea: There are currently two big groups of states: Loading and InGame. Loading does
/// just load assets, but InGame is the actual running game. Every state inside the game is represented
/// as a state on top of InGame in the stack. This enables systems to use InGame for stuff that should always be
/// run when playing the actual game, like updating ui.
fn update_state(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    lives: Res<Lives>,
    state_timer: Option<Res<StateTimer>>,
    assets_loaded_events: EventReader<EAllAssetsLoaded>,
    pacman_hit_events: EventReader<EPacmanHit>,
    edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    ghost_eaten_events: EventReader<EGhostEaten>
) {
    match game_state.current() {
        Loading => switch_to_in_game_when_everything_loaded(&mut game_state, assets_loaded_events),
        InGame => switch_to_start(&mut game_state),
        Start => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 2.0, Ready),
        Ready => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 2.5, Running),
        Running => switch_states_based_on_events(&mut game_state, pacman_hit_events, edibles_eaten_events, ghost_eaten_events),
        PacmanHit => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 1.0, PacmanDying),
        PacmanDying => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 1.5, PacmanDead),
        PacmanDead => switch_to_ready_or_game_over(&mut commands, &state_timer, &lives, &mut game_state),
        GameOver => {} //coming soon!
        LevelTransition => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 3.0, Ready),
        GhostEatenPause => switch_when_timer_finished(&mut commands, &state_timer, &mut game_state, 1.0, Running)
    }
}

fn switch_to_in_game_when_everything_loaded(
    game_state: &mut State<GameState>,
    mut assets_loaded_events: EventReader<EAllAssetsLoaded>
) {
    if assets_loaded_events.iter().count() > 0 {
        game_state.set(InGame).unwrap()
    }
}

/// Executed when the state stack only contains InGame. InGame does effectively nothing,
/// but states over it do. So Start is pushed, as a single InGame should only exist after
/// switching from Loading.
fn switch_to_start(
    game_state: &mut State<GameState>,
) {
    game_state.push(Start).unwrap()
}

fn switch_when_timer_finished(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    game_state: &mut State<GameState>,
    time: f32,
    new_state: GameState
) {
    match state_timer {
        Some(timer) => if timer.finished() {
            commands.remove_resource::<StateTimer>();
            game_state.set(new_state).unwrap();
        },
        None => commands.insert_resource(StateTimer(Timer::from_seconds(time, TimerMode::Once)))
    }
}

fn switch_to_ready_or_game_over(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    lives: &Lives,
    game_state: &mut State<GameState>
) {
    match state_timer {
        Some(timer) => if timer.finished() {
            commands.remove_resource::<StateTimer>();

            if **lives > 0 {
                game_state.set(Ready).unwrap()
            } else {
                game_state.set(GameOver).unwrap()
            }
        },
        None => commands.insert_resource(StateTimer(Timer::from_seconds(1.0, TimerMode::Once)))
    }
}

fn switch_states_based_on_events(
    game_state: &mut State<GameState>,
    mut pacman_hit_events: EventReader<EPacmanHit>,
    mut edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    mut ghost_eaten_events: EventReader<EGhostEaten>
) {
    if pacman_hit_events.iter().count() > 0 {
        game_state.set(PacmanHit).unwrap();
        return;
    }

    if edibles_eaten_events.iter().count() > 0 {
        game_state.set(LevelTransition).unwrap();
        return;
    }

    if ghost_eaten_events.iter().count() > 0 {
        game_state.set(GhostEatenPause).unwrap();
        return;
    }
}

fn update_state_timer(
    time: Res<Time>,
    state_timer: Option<ResMut<StateTimer>>
) {
    if let Some(mut timer) = state_timer {
        timer.tick(time.delta());
    }
}