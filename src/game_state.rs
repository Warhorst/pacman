use bevy::prelude::*;

use crate::game::edibles::EAllEdiblesEaten;
use crate::game::interactions::{GhostWasEaten, PacmanWasHit};
use crate::game::lives::Lives;
use crate::game_assets::EAllAssetsLoaded;
use crate::game_state::Game::*;
use crate::game_state::GameState::*;
use crate::system_sets::ProcessIntersectionsWithPacman;
use crate::ui::game_over_screen::EGameRestarted;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    // TODO there is a bug with an infinite loop when eating a ghost
    //  This is caused by the new set order, where a ghosts state
    //  does not get updated before another hit detection occurs
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(
                Update,
                (
                    update_state
                        .in_set(ProcessIntersectionsWithPacman),
                    update_state_timer
                ),
            )
        ;
    }
}

/// The states of the games state machine.
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    /// The startup state of the game. It will leave Loading when all assets are loaded.
    #[default]
    Loading,
    /// A group of states which represent different phases off the actual game (when you move pacman through the labyrinth)
    Game(Game),
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
    /// Startup state which spawns the labyrinth, ui, etc
    Start,
    /// Phase which spawns pacman, ghosts and the "Ready!" text
    Ready,
    /// The phase of the actual game, when you move around with pacman, eat dots and dodge ghosts.
    Running,
    /// The phase when you were hit by a ghost. Just stops the game for drama.
    PacmanHit,
    /// The phase where the ghosts get despawned and pacman plays his dying animation.
    PacmanDying,
    /// The phase after pacman finished dying. Just another pause for more drama.
    PacmanDead,
    /// The phase that gets entered if pacman died and all lives are lost.
    GameOver,
    /// Short phase where the transition to the next level happens.
    LevelTransition,
    /// A short phase after pacman ate a ghost. A score gets displayed and only already eaten ghosts can move.
    GhostEatenPause,
}

#[derive(Deref, DerefMut, Resource)]
struct StateTimer(Timer);

/// A run condition which returns true if the current state is any variant of Game.
pub fn in_game(current_state: Res<State<GameState>>) -> bool {
    matches!(current_state.get(), Game(_))
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
    pacman_hit_events: EventReader<PacmanWasHit>,
    edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    ghost_eaten_events: EventReader<GhostWasEaten>,
    game_restartet_events: EventReader<EGameRestarted>,
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
    mut assets_loaded_events: EventReader<EAllAssetsLoaded>,
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
    new_state: GameState,
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
    game_state: &mut NextState<GameState>,
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
    mut pacman_hit_events: EventReader<PacmanWasHit>,
    mut edibles_eaten_events: EventReader<EAllEdiblesEaten>,
    mut ghost_eaten_events: EventReader<GhostWasEaten>,
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
    mut game_restarted_events: EventReader<EGameRestarted>,
) {
    if game_restarted_events.iter().count() > 0 {
        game_state.set(Game(Start))
    }
}

fn update_state_timer(
    time: Res<Time>,
    state_timer: Option<ResMut<StateTimer>>,
) {
    if let Some(mut timer) = state_timer {
        timer.tick(time.delta());
    }
}