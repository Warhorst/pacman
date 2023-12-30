use bevy::prelude::*;
use pad::Position;
use crate::core::edibles::EdiblesPlugin;
use crate::core::pacman::PacmanPlugin;
use crate::prelude::*;

pub mod position;
pub mod direction;
pub mod edibles;
pub mod pacman;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Pos>()
            .register_type::<Position>()
            .register_type::<Dir>()
            .add_plugins((
                PacmanPlugin,
                EdiblesPlugin,
            ))
        ;
    }
}
