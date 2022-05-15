use bevy::prelude::*;
use bevy::utils::Duration;

use self::State::*;

pub(super) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ScheduleChanged>()
            .insert_resource(create_default_schedule())
            .add_system(update_schedule);
    }
}

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
        self.current_phase().active_state()
    }

    pub fn current_state(&self) -> State {
        self.current_phase().active_state()
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
pub enum Phase {
    Finite(State, Timer),
    Infinite(State)
}

impl Phase {
    pub fn for_duration(active_state: State, duration: f32) -> Self {
        Phase::Finite(active_state, Timer::from_seconds(duration, false))
    }

    pub fn infinite(active_state: State) -> Self {
        Phase::Infinite(active_state)
    }

    pub fn active_state(&self) -> State {
        match self {
            Phase::Finite(s, _) => *s,
            Phase::Infinite(s) => *s
        }
    }

    pub fn progress(&mut self, elapsed_time: Duration) {
        if let Phase::Finite(_, ref mut timer) = self {
            timer.tick(elapsed_time);
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Phase::Finite(_, timer) => timer.finished(),
            Phase::Infinite(_) => false
        }
    }
}

#[derive(Deref, DerefMut)]
pub(super) struct ScheduleChanged(State);

/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum State {
    ChaseState,
    ScatterState,
}

fn update_schedule(
    time: Res<Time>,
    mut schedule: ResMut<Schedule>,
    mut event_writer: EventWriter<ScheduleChanged>,
) {
    let old_state = schedule.current_state();
    let new_state = schedule.state_after_tick(time.delta());

    if old_state != new_state {
        event_writer.send(ScheduleChanged(new_state))
    }
}

fn create_default_schedule() -> Schedule {
    let mut phases = Vec::new();
    phases.push(Phase::for_duration(ScatterState, 7.0));
    phases.push(Phase::for_duration(ChaseState, 20.0));
    phases.push(Phase::for_duration(ScatterState, 7.0));
    phases.push(Phase::for_duration(ChaseState, 20.0));
    phases.push(Phase::for_duration(ScatterState, 5.0));
    phases.push(Phase::for_duration(ChaseState, 20.0));
    phases.push(Phase::for_duration(ScatterState, 5.0));
    phases.push(Phase::infinite(ChaseState));
    Schedule::new(phases)
}

