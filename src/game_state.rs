use bevy::prelude::*;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::edibles::EAllEdiblesEaten;
use crate::game::interactions::{EGhostEaten, EPacmanHit};
use crate::game::lives::Lives;
use crate::game_assets::EAllAssetsLoaded;
use crate::ui::game_over_screen::EGameRestarted;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(Update, (
                update_state,
                update_state_timer
            ))
        ;
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Loading,
    Game(Game)
}

impl States for GameState {
    type Iter = std::array::IntoIter<GameState, 10>;

    fn variants() -> Self::Iter {
        [
            Loading,
            Game(Start),
            Game(Ready),
            Game(Running),
            Game(PacmanHit),
            Game(PacmanDying),
            Game(PacmanDead),
            Game(GameOver),
            Game(LevelTransition),
            Game(GhostEatenPause)
        ].into_iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Game {
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

/// A run condition which returns true if the current state is any variant of Game.
pub fn in_game() -> impl Fn(Res<State<GameState>>) -> bool {
    |current_state: Res<State<GameState>>| match current_state.get() {
        Game(_) => true,
        _ => false
    }
}

/// Update the current game state based on multiple factors.
///
/// The idea: There are currently two big groups of states: Loading and InGame. Loading does
/// just load assets, but InGame is the actual running game. Every state inside the game is represented
/// as a state on top of InGame in the stack. This enables systems to use InGame for stuff that should always be
/// run when playing the actual game, like updating ui.
fn update_state(
    mut commands: Commands,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    lives: Res<Lives>,
    state_timer: Option<Res<StateTimer>>,
    assets_loaded_events: EventReader<EAllAssetsLoaded>,
    pacman_hit_events: EventReader<EPacmanHit>,
    edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    ghost_eaten_events: EventReader<EGhostEaten>,
    game_restartet_events: EventReader<EGameRestarted>
) {
    match current_state.get() {
        Loading => switch_to_in_game_when_everything_loaded(&mut next_state, assets_loaded_events),
        Game(Start) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 2.0, Game(Ready)),
        Game(Ready) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 2.5, Game(Running)),
        Game(Running) => switch_states_based_on_events(&mut next_state, pacman_hit_events, edibles_eaten_events, ghost_eaten_events),
        Game(PacmanHit) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 1.0, Game(PacmanDying)),
        Game(PacmanDying) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 1.5, Game(PacmanDead)),
        Game(PacmanDead) => switch_to_ready_or_game_over(&mut commands, &state_timer, &lives, &mut next_state),
        Game(GameOver) => switch_to_start_after_game_over(&mut next_state, game_restartet_events),
        Game(LevelTransition) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 3.0, Game(Ready)),
        Game(GhostEatenPause) => switch_when_timer_finished(&mut commands, &state_timer, &mut next_state, 1.0, Game(Running))
    }
}

fn switch_to_in_game_when_everything_loaded(
    game_state: &mut NextState<GameState>,
    mut assets_loaded_events: EventReader<EAllAssetsLoaded>
) {
    if assets_loaded_events.iter().count() > 0 {
        game_state.set(Game(Start))
    }
}

fn switch_when_timer_finished(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    game_state: &mut NextState<GameState>,
    time: f32,
    new_state: GameState
) {
    match state_timer {
        Some(timer) => if timer.finished() {
            commands.remove_resource::<StateTimer>();
            game_state.set(new_state);
        },
        None => commands.insert_resource(StateTimer(Timer::from_seconds(time, TimerMode::Once)))
    }
}

fn switch_to_ready_or_game_over(
    commands: &mut Commands,
    state_timer: &Option<Res<StateTimer>>,
    lives: &Lives,
    game_state: &mut NextState<GameState>
) {
    match state_timer {
        Some(timer) => if timer.finished() {
            commands.remove_resource::<StateTimer>();

            if **lives > 0 {
                game_state.set(Game(Ready))
            } else {
                game_state.set(Game(GameOver))
            }
        },
        None => commands.insert_resource(StateTimer(Timer::from_seconds(1.0, TimerMode::Once)))
    }
}

fn switch_states_based_on_events(
    game_state: &mut NextState<GameState>,
    mut pacman_hit_events: EventReader<EPacmanHit>,
    mut edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    mut ghost_eaten_events: EventReader<EGhostEaten>
) {
    if pacman_hit_events.iter().count() > 0 {
        game_state.set(Game(PacmanHit));
        return;
    }

    if edibles_eaten_events.iter().count() > 0 {
        game_state.set(Game(LevelTransition));
        return;
    }

    if ghost_eaten_events.iter().count() > 0 {
        game_state.set(Game(GhostEatenPause));
        return;
    }
}

fn switch_to_start_after_game_over(
    game_state: &mut NextState<GameState>,
    mut game_restarted_events: EventReader<EGameRestarted>
) {
    if game_restarted_events.iter().count() > 0 {
        game_state.set(Game(Start))
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