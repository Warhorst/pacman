use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::ghosts::components::Ghost;
use crate::ghosts::state::State;
use crate::ghosts::target::Target;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(DebugInfoTimer::new())
            .add_system(print_debug_data.system());
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
                    ghost_query: Query<(&Ghost, &Position, &Target, &Movement, &State)>) {
    debug_timer.timer.tick(time.delta());
    if !debug_timer.timer.finished() {
        return;
    }

    println!();

    for (ghost, position, target, movement, state) in ghost_query.iter() {
        println!("{}",
                 format!("Ghost: {:?}, position: {:?}, target: {:?}, movement: {:?}, state: {:?}",
                         ghost,
                         position,
                         target.get_position_opt(),
                         movement,
                         state
                 )
        )
    }
}