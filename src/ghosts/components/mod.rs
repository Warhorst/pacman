use bevy::prelude::Timer;

use crate::common::Position;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Target {
    target: Option<Position>
}

impl Target {
    pub fn new() -> Self {
        Target { target: None }
    }

    pub fn is_set(&self) -> bool {
        self.target.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    pub fn set_to(&mut self, position: Position) {
        self.target = Some(position)
    }

    pub fn get_position(&self) -> &Position {
        &self.target.as_ref().expect("The target should be set at this point")
    }

    pub fn get_position_opt(&self) -> &Option<Position> {
        &self.target
    }

    pub fn clear(&mut self) {
        self.target = None
    }
}

/// The different states of a ghost
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum State {
    Spawned,
    Chase,
    Scatter,
    Eaten,
    Frightened,
}

/// The schedule of a ghost determines the state the ghost has after a certain time passed
/// on a certain level.
/// The last phase of a schedule will be active until the level ends, even if its timer is finished.
pub struct Schedule {
    phases: Vec<Phase>
}

impl Schedule {
    pub fn new(phases: Vec<Phase>) -> Self {
        Schedule { phases }
    }

    pub fn state_after_tick(&mut self, delta_time: f32) -> State {
        self.current_phase().progress(delta_time);
        if self.current_phase().is_finished() {
            self.start_next_phase()
        }
        self.current_phase().active_state
    }

    fn current_phase(&mut self) -> &mut Phase {
        &mut self.phases[0]
    }

    fn start_next_phase(&mut self) {
        if self.phases.len() > 1 {
            self.phases.remove(0);
        }
    }
}

/// A Phase is a time range where a specific state for a specific ghost is active.
pub struct Phase {
    active_state: State,
    remaining_time: Timer
}

impl Phase {
    pub fn new(active_state: State, duration: f32) -> Self {
        Phase {
            active_state,
            remaining_time : Timer::from_seconds(duration, false)
        }
    }

    pub fn progress(&mut self, delta_time: f32) {
        self.remaining_time.tick(delta_time);
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_time.finished()
    }
}