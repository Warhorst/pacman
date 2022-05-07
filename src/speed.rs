use bevy::prelude::*;
use crate::constants::GHOST_SPEED;
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::ghosts::state::State::Frightened;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_ghost_speed_when_state_changed)
        ;
    }
}

/// The current speed of a moving entity
#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

// TODO: I try out change detection with this one. to keep the app consistent,
//  a "way to go" (events or change detection) should be choosen for each system.
//  Currently, it's mostly events.
fn update_ghost_speed_when_state_changed(
    mut query: Query<(&mut Speed, &State), (With<Ghost>, Changed<State>)>
) {
    for (mut speed, state) in query.iter_mut() {
        match state {
            Frightened => **speed *= 0.5,
            _ => **speed = GHOST_SPEED
        }
    }
}