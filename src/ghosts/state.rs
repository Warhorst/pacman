use bevy::prelude::*;
use crate::ghosts::State;
use std::thread::current;
use crate::level::Level;
use std::collections::HashMap;

pub struct StateChanger {

}

impl StateChanger {

}

/// A schedule of phases.
/// Returns the state of the currently active phase.
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

    /// Update the schedule and return the currently active state.
    pub fn update_and_get_state(&mut self, delta_seconds: f32) -> State {
        if self.current_phase().finished_after_update(delta_seconds) {
            self.switch_to_next_phase();
        }
        self.current_phase().active_state
    }

    fn current_phase(&mut self) -> &mut Phase {
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
        match self.timer.finished {
            false => {
                self.timer.tick(delta_seconds);
                self.timer.finished
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
    use super::Phase;
    use crate::ghosts::State::*;
    use crate::ghosts::state::Schedule;
    use crate::level::Level;

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
        assert_eq!(Spawned, schedule.update_and_get_state(1.0));
        assert_eq!(Scatter, schedule.update_and_get_state(1.0));
    }

    /// The last phase should be active when the previous ended, the last
    /// phase ended and the time of the last phase was exceeded.
    #[test]
    fn schedule_last_phase_stays_forever() {
        let mut schedule = Schedule::new(Level::new(1), vec![
            Phase::new(1.0, Spawned),
            Phase::last(Scatter)
        ]);
        assert_eq!(Scatter, schedule.update_and_get_state(1.0));
        assert_eq!(Scatter, schedule.update_and_get_state(1.0));
        assert_eq!(Scatter, schedule.update_and_get_state(1.0));
    }

    #[test]
    fn schedule_reset_correctly() {
        let mut schedule = Schedule::new(Level::new(1), vec![
            Phase::new(2.0, Spawned),
            Phase::last(Scatter)
        ]);
        assert_eq!(Scatter, schedule.update_and_get_state(3.0));
        schedule.reset();
        assert_eq!(Spawned, schedule.update_and_get_state(1.0));
    }
}