use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::ghosts::Ghost;
use crate::ghosts::state::State;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugInfoTimer::new())
            .add_system(print_debug_data);
    }
}

struct DebugInfoTimer {
    pub timer: Timer
}

impl DebugInfoTimer {
    pub fn new() -> Self {
        DebugInfoTimer {
            timer: Timer::from_seconds(1.0, true)
        }
    }
}

fn print_debug_data(mut debug_timer: ResMut<DebugInfoTimer>,
                    time: Res<Time>,
                    ghost_query: Query<(&Ghost, &Position, &Movement, &State)>) {
    debug_timer.timer.tick(time.delta());
    if !debug_timer.timer.finished() {
        return;
    }

    println!();

    for (ghost, position, movement, state) in ghost_query.iter() {
        println!("{}",
                 format!("Ghost: {:?}, position: {:?}, movement: {:?}, state: {:?}",
                         ghost,
                         position,
                         movement,
                         state
                 )
        )
    }
}