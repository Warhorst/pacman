use bevy::prelude::*;
use crate::game::edibles::dots::DotPlugin;
use crate::game::edibles::energizer::EnergizerPlugin;
use crate::game::edibles::fruit::FruitPlugin;
use crate::core::prelude::*;

pub mod dots;
pub mod fruit;
pub mod energizer;

pub(in crate::game) struct EdiblePlugin;

impl Plugin for EdiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DotPlugin,
                EnergizerPlugin,
                FruitPlugin
            ))
            .add_systems(Update, check_if_all_edibles_eaten.run_if(in_state(Game(Running))))
        ;
    }
}

fn check_if_all_edibles_eaten(
    mut event_writer: EventWriter<EAllEdiblesEaten>,
    query: Query<&Edible>,
) {
    if query.iter().count() == 0 {
        event_writer.send(EAllEdiblesEaten)
    }
}