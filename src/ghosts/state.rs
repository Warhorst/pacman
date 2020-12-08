use bevy::prelude::*;
use crate::ghosts::State;
use std::thread::current;
use crate::level::Level;
use std::collections::HashMap;

pub struct StateChanger {

}

impl StateChanger {

}

pub struct Schedule {
    level: Level,
    phases: Vec<Phase>,
    current_phase_index: usize
}

/// A schedule of phases.
/// When one phase is over, switch to the next one, returning
/// the next state.
impl Schedule {
    pub fn new(level: Level, mut phases: Vec<Phase>) -> Self {
        Schedule {
            level,
            phases,
            current_phase_index: 0
        }
    }

    /// Update the current phase with the given delta seconds.
    /// If the phases is finished, switch to the next.
    pub fn update_and_get_next(&mut self, delta_seconds: f32) -> Option<State> {
        match self.current_phase().update_and_get_next(delta_seconds) {
            None => None,
            Some(state) => {
                self.switch_to_next_phase();
                Some(state)
            }
        }
    }

    fn current_phase(&mut self) -> &mut Phase {
        &mut self.phases[self.current_phase_index]
    }

    fn switch_to_next_phase(&mut self) {
        if self.current_phase_index < self.phases.len() - 1 {
            self.current_phase_index += 1
        }
    }

    fn reset(&mut self) {
        self.phases.iter_mut()
            .for_each(|phase| phase.reset());
        self.current_phase_index = 0
    }
}

/// A time range where a specific state is active.
/// When the timer is finished, switch to the next state.
pub struct Phase {
    timer: Timer,
    next_state: State,
}

impl Phase {
    fn new(duration_in_seconds: f32, next_state: State) -> Self {
        Phase {
            timer: Timer::from_seconds(duration_in_seconds, false),
            next_state,
        }
    }

    /// Ticks the timer with the received delta seconds.
    /// If the timer reached its end, return the next state.
    /// If not, return None.
    fn update_and_get_next(&mut self, delta_seconds: f32) -> Option<State> {
        match self.timer.finished {
            false => {
                self.timer.tick(delta_seconds);
                None
            },
            true => Some(self.next_state)
        }
    }

    fn reset(&mut self) {
        self.timer.reset()
    }
}