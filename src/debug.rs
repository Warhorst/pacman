use bevy::prelude::*;

use crate::common::{MoveDirection, Position};
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
    pub timer: Timer,
}

impl DebugInfoTimer {
    pub fn new() -> Self {
        DebugInfoTimer {
            timer: Timer::from_seconds(1.0, true)
        }
    }
}

fn print_debug_data(
    mut debug_timer: ResMut<DebugInfoTimer>,
    time: Res<Time>,
    ghost_query: Query<(&Ghost, &Position, &MoveDirection, &State)>,
) {
    debug_timer.timer.tick(time.delta());
    if !debug_timer.timer.finished() {
        return;
    }

    println!();

    for (ghost, position, direction, state) in ghost_query.iter() {
        println!("{}",
                 format!("Ghost: {:?}, position: {:?}, movement: {:?}, state: {:?}",
                         ghost,
                         position,
                         direction,
                         state
                 )
        )
    }
}