use bevy::prelude::*;

pub(super) struct RestartGamePlugin;

impl Plugin for RestartGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameWasRestarted>()
            .register_type::<GameWasRestarted>()
        ;
    }
}

/// Event which is sent when the player decided to restart the game
#[derive(Event, Reflect)]
pub struct GameWasRestarted;