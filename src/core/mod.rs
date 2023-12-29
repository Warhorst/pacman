use bevy::prelude::*;
use pad::Position;
use crate::prelude::*;

pub mod position;
pub mod direction;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Pos>()
            .register_type::<Position>()
            .register_type::<Dir>()
        ;
    }
}
