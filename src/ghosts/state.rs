use std::collections::HashMap;
use std::thread::current;

use bevy::prelude::*;

use crate::ghosts::State;
use crate::ghosts::State::*;
use crate::level::Level;

pub struct ActiveState {
    current_state: State,
    schedule: Schedule,
    schedule_paused: bool
}

impl ActiveState {
    pub fn new(schedule: Schedule) -> Self {
        ActiveState {
            current_state: schedule.get_active_state(),
            schedule,
            schedule_paused: false
        }
    }

    pub fn set_eaten(&mut self) {
        self.set_off_schedule_state(Eaten)
    }

    pub fn set_frightened(&mut self) {
        self.set_off_schedule_state(Frightened)
    }

    fn set_off_schedule_state(&mut self, state: State) {
        self.current_state = state;
        self.schedule_paused = true
    }
}

/// A schedule of states a ghost should accept in a specific
/// order and after a specific time.
pub struct Schedule {
    level: Level,
    phases: Vec<Phase>,
    current_phase_index: usize
}

impl Schedule {
    pub fn new(level: Level, mut phases: Vec<Phase>) -> Self {
        Schedule {
            level,
            phases,
            current_phase_index: 0
        }
    }

    pub fn get_active_state(&self) -> State {
        self.current_phase().active_state
    }

    /// Update the schedule and returns if the current phase was finished.
    /// Switches the phase if necessary.
    pub fn finished_after_update(&mut self, delta_seconds: f32) -> bool {
        if self.current_phase_mut().finished_after_update(delta_seconds) {
            self.switch_to_next_phase();
            return true
        }
        false
    }

    fn current_phase(&self) -> &Phase {
        &self.phases[self.current_phase_index]
    }

    fn current_phase_mut(&mut self) -> &mut Phase {
        &mut self.phases[self.current_phase_index]
    }

    /// Switch to the next phase (if not currently on the last phase).
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
    active_state: State,
}

impl Phase {
    fn new(duration_in_seconds: f32, active_state: State) -> Self {
        Phase {
            timer: Timer::from_seconds(duration_in_seconds, false),
            active_state,
        }
    }

    /// Creates a phase which is intended to be a last phase of a schedule.
    /// The duration of a last phase doesn't matter, so it is set to zero.
    /// This way, calls to Phase::new with useless times aren't necessary.
    fn last(active_state: State) -> Self {
        Phase::new(0.0, active_state)
    }

    /// Updates this phase and returns true if it ended
    fn finished_after_update(&mut self, delta_seconds: f32) -> bool {
        match self.timer.finished() {
            false => {
                self.timer.tick(delta_seconds);
                self.timer.finished()
            },
            true => true
        }
    }

    fn reset(&mut self) {
        self.timer.reset()
    }
}

#[cfg(test)]
mod tests {
    use crate::ghosts::State::*;
    use crate::ghosts::state::Schedule;
    use crate::level::Level;

    use super::Phase;

    #[test]
    fn phase_finished_after_set_time() {
        let mut phase = Phase::new(2.0, Scatter);
        assert_eq!(false, phase.finished_after_update(1.0));
        assert_eq!(true, phase.finished_after_update(1.0));
    }

    #[test]
    fn last_phase_just_finished() {
        let mut phase = Phase::last(Scatter);
        assert_eq!(true, phase.finished_after_update(0.0));
    }

    #[test]
    fn phase_reset_correctly() {
        let mut phase = Phase::new(2.0, Scatter);
        assert_eq!(true, phase.finished_after_update(2.0));
        phase.reset();
        assert_eq!(false, phase.finished_after_update(1.0))
    }

    #[test]
    fn schedule_phase_switched() {
        let mut schedule = Schedule::new(Level::new(1), vec![
            Phase::new(2.0, Spawned),
            Phase::last(Scatter)
        ]);
        assert_eq!(false, schedule.finished_after_update(1.0));
        assert_eq!(Spawned, schedule.get_active_state());
        assert_eq!(true, schedule.finished_after_update(1.0));
        assert_eq!(Scatter, schedule.get_active_state());
    }

    /// The last phase should be active when the previous ended, the last
    /// phase ended and the time of the last phase was exceeded.
    #[test]
    fn schedule_last_phase_stays_forever() {
        let mut schedule = Schedule::new(Level::new(1), vec![
            Phase::new(1.0, Spawned),
            Phase::last(Scatter)
        ]);
        assert_eq!(true, schedule.finished_after_update(1.0));
        assert_eq!(Scatter, schedule.get_active_state());
        assert_eq!(true, schedule.finished_after_update(1.0));
        assert_eq!(Scatter, schedule.get_active_state());
        assert_eq!(true, schedule.finished_after_update(1.0));
        assert_eq!(Scatter, schedule.get_active_state());
    }

    #[test]
    fn schedule_reset_correctly() {
        let mut schedule = Schedule::new(Level::new(1), vec![
            Phase::new(2.0, Spawned),
            Phase::last(Scatter)
        ]);
        assert_eq!(true, schedule.finished_after_update(3.0));
        assert_eq!(Scatter, schedule.get_active_state());
        schedule.reset();
        assert_eq!(false, schedule.finished_after_update(1.0));
        assert_eq!(Spawned, schedule.get_active_state());
    }
}