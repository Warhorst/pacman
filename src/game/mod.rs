use bevy::prelude::*;
use crate::game::edibles::EdiblePlugin;
use crate::game::ghost_house_gate::GhostHouseGatePlugin;
use crate::game::ghosts::GhostPlugin;
use crate::game::interactions::InteractionsPlugin;
use crate::game::level::LevelPlugin;
use crate::game::lives::LivesPlugin;
use crate::game::map::MapPlugin;
use crate::game::pacman::PacmanPlugin;
use crate::game::random::RandomPlugin;
use crate::game::schedule::SchedulePlugin;
use crate::game::score::ScorePlugin;
use crate::game::specs_per_level::SpecsPerLevelPlugin;
use crate::game::speed::SpeedPlugin;
use crate::game::state::StatePlugin;
use crate::game::target::TargetPlugin;

pub mod interactions;
pub mod random;
pub mod score;
pub mod speed;
pub mod specs_per_level;
pub mod lives;
pub mod level;
pub mod ghost_house_gate;
pub mod map;
pub mod edibles;
pub mod pacman;
pub mod ghosts;
pub mod direction;
pub mod helper;
mod schedule;
pub mod state;
pub mod target;
mod position;

/// Contains the entire gameplay logic for pacman.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EdiblePlugin,
                GhostPlugin,
                GhostHouseGatePlugin,
                InteractionsPlugin,
                LevelPlugin,
                LivesPlugin,
                MapPlugin,
                PacmanPlugin,
                RandomPlugin,
                SchedulePlugin,
                ScorePlugin,
                SpecsPerLevelPlugin,
                SpeedPlugin,
                StatePlugin,
                TargetPlugin
            ))
        ;
    }
}
