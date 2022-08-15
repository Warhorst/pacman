use std::ops::RangeInclusive;
use bevy::prelude::*;
use bevy::utils::Duration;
use crate::edibles::energizer::EnergizerTimer;
use crate::life_cycle::LifeCycle::*;
use crate::level::Level;
use crate::ghosts::state::State;
use crate::ghosts::state::State::*;
use crate::life_cycle::LifeCycle::Start;

pub(super) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ScheduleChanged>()
            .insert_resource(create_schedules())
            .add_system_set(
                SystemSet::on_enter(Start).with_system(register_start_schedule)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(update_schedule_when_level_changed)
                    .with_system(update_schedule)
            )
        ;
    }
}

fn create_schedules() -> ScheduleByLevel {
    ScheduleByLevel::new(vec![
        LevelScheduleRange::new(Level(1)..=Level(1), level_one_schedule),
        LevelScheduleRange::new(Level(2)..=Level(4), level_two_to_four_schedule),
        LevelScheduleRange::new(Level(5)..=Level(usize::MAX), level_five_plus_schedule),
    ])
}

fn register_start_schedule(
    mut commands: Commands,
    level: Res<Level>,
    schedule_by_level: Res<ScheduleByLevel>,
) {
    commands.insert_resource(schedule_by_level.get_schedule_for_level(&level));
}

fn update_schedule_when_level_changed(
    mut schedule: ResMut<Schedule>,
    level: Res<Level>,
    schedule_by_level: Res<ScheduleByLevel>,
) {
    if !level.is_changed() { return; }

    *schedule = schedule_by_level.get_schedule_for_level(&level);
}

/// Update the currently active schedule and send an event when the active state changed.
///
/// The schedule does not proceed while an energizer is active.
fn update_schedule(
    time: Res<Time>,
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut schedule: ResMut<Schedule>,
    mut event_writer: EventWriter<ScheduleChanged>,
) {
    if energizer_timer.is_some() { return; }

    let old_state = schedule.current_state();
    let new_state = schedule.state_after_tick(time.delta());

    if old_state != new_state {
        event_writer.send(ScheduleChanged(new_state))
    }
}

fn level_one_schedule() -> Schedule {
    let mut phases = Vec::new();
    phases.push(Phase::for_duration(Scatter, 7.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 7.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::for_duration(Chase, 1033.0));
    phases.push(Phase::for_duration(Scatter, 1.0 / 60.0));
    phases.push(Phase::infinite(Chase));
    Schedule::new(phases)
}

fn level_two_to_four_schedule() -> Schedule {
    let mut phases = Vec::new();
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::for_duration(Chase, 1037.0));
    phases.push(Phase::for_duration(Scatter, 1.0 / 60.0));
    phases.push(Phase::infinite(Chase));
    Schedule::new(phases)
}

fn level_five_plus_schedule() -> Schedule {
    let mut phases = Vec::new();
    phases.push(Phase::for_duration(Scatter, 7.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 7.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::for_duration(Chase, 20.0));
    phases.push(Phase::for_duration(Scatter, 5.0));
    phases.push(Phase::infinite(Chase));
    Schedule::new(phases)
}

/// Indicates that a new state is active
#[derive(Deref, DerefMut)]
pub struct ScheduleChanged(State);

pub struct ScheduleByLevel {
    ranges: Vec<LevelScheduleRange>,
}

impl ScheduleByLevel {
    pub fn new(ranges: Vec<LevelScheduleRange>) -> Self {
        ScheduleByLevel { ranges }
    }

    pub fn get_schedule_for_level(&self, level: &Level) -> Schedule {
        self.ranges.iter()
            .find_map(|r| match r.contains_level(level) {
                true => Some((r.schedule_producer)()),
                false => None
            })
            .expect("No schedule was registered for the current level")
    }
}

pub struct LevelScheduleRange {
    range: RangeInclusive<Level>,
    schedule_producer: Box<dyn Fn() -> Schedule + Send + Sync>,
}

impl LevelScheduleRange {
    pub fn new(range: RangeInclusive<Level>, schedule_producer: impl Fn() -> Schedule + 'static + Send + Sync) -> Self {
        LevelScheduleRange {
            range,
            schedule_producer: Box::new(schedule_producer),
        }
    }

    pub fn contains_level(&self, level: &Level) -> bool {
        self.range.contains(level)
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
    Infinite(State),
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