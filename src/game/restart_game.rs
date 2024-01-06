use bevy::prelude::*;
use crate::core::prelude::*;

pub(super) struct RestartGamePlugin;

impl Plugin for RestartGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                send_restart_event_on_key_press.run_if(in_state(Game(GameOver)))
            )
        ;
    }
}



fn send_restart_event_on_key_press(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<GameWasRestarted>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        event_writer.send(GameWasRestarted);
    }
}