use bevy::prelude::*;
use bevy::utils::Duration;

use crate::ghosts::state::State;
use crate::ghosts::state::State::*;

/// The schedule of a ghost determines the state the ghost has after a certain time passed
/// on a certain level.
/// The last phase of a schedule will be active until the level ends, even if its timer is finished.
pub struct Schedule {
    phases: Vec<Phase>,
}

impl Schedule {
    pub fn new(phases: Vec<Phase>) -> Self {
        Schedule { phases }
    }

    pub fn state_after_tick(&mut self, elapsed_time: Duration) -> State {
        self.current_phase_mut().progress(elapsed_time);
        if self.current_phase().is_finished() {
            self.start_next_phase()
        }
        self.current_phase().active_state
    }

    pub fn current_state(&self) -> State {
        self.current_phase().active_state
    }

    fn current_phase(&self) -> &Phase {
        &self.phases[0]
    }

    fn current_phase_mut(&mut self) -> &mut Phase {
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
    remaining_time: Timer,
}

impl Phase {
    pub fn new(active_state: State, duration: f32) -> Self {
        Phase {
            active_state,
            remaining_time: Timer::from_seconds(duration, false),
        }
    }

    pub fn progress(&mut self, elapsed_time: Duration) {
        self.remaining_time.tick(elapsed_time);
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_time.finished()
    }
}

pub(super) struct PhaseChanged;

pub(super) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<PhaseChanged>()
            .insert_resource(create_default_schedule())
            .add_system(update_schedule.system());
    }
}

fn update_schedule(
    time: Res<Time>,
    mut schedule: ResMut<Schedule>,
    mut event_writer: EventWriter<PhaseChanged>,
) {
    let old_state = schedule.current_state();
    let new_state = schedule.state_after_tick(time.delta());

    if old_state != new_state {
        event_writer.send(PhaseChanged)
    }
}

fn create_default_schedule() -> Schedule {
    let mut phases = Vec::new();
    phases.push(Phase::new(Scatter, 10.0));
    phases.push(Phase::new(Chase, 10.0));
    phases.push(Phase::new(Scatter, 10.0));
    Schedule::new(phases)
}

