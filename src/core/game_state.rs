use bevy::prelude::*;
use crate::prelude::*;

pub(super) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GameState>()
        ;
    }
}

/// The states of the games state machine.
#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    /// Perform necessary setup steps before the game can start
    Setup(Setup),
    /// Spawn the maze
    Spawn(Spawn),
    /// A group of states which represent different phases off the actual game (when you move pacman through the labyrinth)
    Game(Game),
}

impl Default for GameState {
    fn default() -> Self {
        Setup(PreloadAssets)
    }
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Setup {
    /// Start the preload of all assets of the game
    PreloadAssets,
    /// Create all sprite sheets from the preloaded assets
    CreateSpriteSheets
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Spawn {
    /// Load the map scene and spawn everything from it
    SpawnMapScene,
    /// Enhance the spawned entities with textures and more
    EnhanceMap,
    /// Spawn the ui of the game
    SpawnUi
}

#[derive(Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

/// A run condition which returns true if the current state is any variant of Game.
pub fn in_game(current_state: Res<State<GameState>>) -> bool {
    matches!(current_state.get(), Game(_))
}