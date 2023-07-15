use bevy::prelude::*;
use crate::game::edibles::dots::DotPlugin;
use crate::game::edibles::energizer::EnergizerPlugin;
use crate::game::edibles::fruit::FruitPlugin;
use crate::game_state::GameState::Running;

pub mod dots;
pub mod fruit;
pub mod energizer;

pub(in crate::game) struct EdiblePlugin;

impl Plugin for EdiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EAllEdiblesEaten>()
            .add_plugins((
                DotPlugin,
                EnergizerPlugin,
                FruitPlugin
            ))
            .add_systems(Update, check_if_all_edibles_eaten.run_if(in_state(Running)))
        ;
    }
}

/// Component for everything in the maze that is edible.
#[derive(Component)]
pub struct Edible;

/// Event that gets fired when all edibles are eaten (or at least gone), so the maze is empty
#[derive(Event)]
pub struct EAllEdiblesEaten;

fn check_if_all_edibles_eaten(
    mut event_writer: EventWriter<EAllEdiblesEaten>,
    query: Query<&Edible>,
) {
    if query.iter().count() == 0 {
        event_writer.send(EAllEdiblesEaten)
    }
}