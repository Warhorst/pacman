use bevy::prelude::*;
use bevy::utils::{Duration, HashMap};
use crate::game::edibles::energizer::EnergizerTimer;
use crate::game_state::GameState::*;
use crate::game::level::Level;
use crate::game::state::State;
use crate::game::state::State::*;

pub(in crate::game) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScheduleByLevel::new())
            .add_system_set(
                SystemSet::on_enter(Start).with_system(register_start_schedule)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(switch_schedule_when_level_changed)
                    .with_system(update_schedule)
            )
        ;
    }
}

fn register_start_schedule(
    mut commands: Commands,
    schedule_by_level: Res<ScheduleByLevel>,
) {
    commands.insert_resource(schedule_by_level.get_schedule_for_level(&Level(1)));
}

fn switch_schedule_when_level_changed(
    mut schedule: ResMut<Schedule>,
    level: Res<Level>,
    schedule_by_level: Res<ScheduleByLevel>,
) {
    if !level.is_changed() { return; }

    *schedule = schedule_by_level.get_schedule_for_level(&level);
}

/// Update the currently active schedule.
///
/// The schedule does not proceed while an energizer is active.
fn update_schedule(
    time: Res<Time>,
    energizer_timer: Option<Res<EnergizerTimer>>,
    mut schedule: ResMut<Schedule>,
) {
    if energizer_timer.is_none() {
        schedule.update(time.delta());
    }
}

#[derive(Resource)]
pub struct ScheduleByLevel {
    level_schedule_map: HashMap<Level, Schedule>,
    default_schedule: Schedule,
}

impl ScheduleByLevel {
    fn new() -> Self {
        ScheduleByLevel {
            level_schedule_map: [
                (Level(1), Self::level_one()),
                (Level(2), Self::level_two_to_four()),
                (Level(3), Self::level_two_to_four()),
                (Level(4), Self::level_two_to_four()),
            ].into_iter().collect(),
            default_schedule: Self::level_five_and_beyond(),
        }
    }

    fn level_one() -> Schedule {
        Schedule::new([
            Phase::for_seconds(Scatter, 7.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 7.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 5.0),
            Phase::for_seconds(Chase, 1033.0),
            Phase::for_seconds(Scatter, 1.0 / 60.0),
            Phase::infinite(Chase)
        ])
    }

    fn level_two_to_four() -> Schedule {
        Schedule::new([
            Phase::for_seconds(Scatter, 5.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 5.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 5.0),
            Phase::for_seconds(Chase, 1037.0),
            Phase::for_seconds(Scatter, 1.0 / 60.0),
            Phase::infinite(Chase)
        ])
    }

    fn level_five_and_beyond() -> Schedule {
        Schedule::new([
            Phase::for_seconds(Scatter, 7.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 7.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 5.0),
            Phase::for_seconds(Chase, 20.0),
            Phase::for_seconds(Scatter, 5.0),
            Phase::infinite(Chase),
        ])
    }

    pub fn get_schedule_for_level(&self, level: &Level) -> Schedule {
        self.level_schedule_map.get(level).unwrap_or(&self.default_schedule).clone()
    }
}

#[derive(Clone, Resource)]
pub struct Schedule {
    current_phase_index: usize,
    current_phase_timer: Option<Timer>,
    phases: Vec<Phase>,
}

impl Schedule {
    fn new(phases: impl IntoIterator<Item=Phase>) -> Self {
        let phases = phases.into_iter().collect::<Vec<_>>();

        Schedule {
            current_phase_index: 0,
            current_phase_timer: phases.get(0).expect("at least one phase must be provided").phase_timer(),
            phases,
        }
    }

    pub fn current_state(&self) -> State {
        self.phases[self.current_phase_index].state
    }

    pub fn update(&mut self, delta: Duration) {
        if let Some(ref mut timer) = self.current_phase_timer {
            timer.tick(delta);

            if timer.finished() {
                self.switch_to_next_phase()
            }
        }
    }

    fn switch_to_next_phase(&mut self) {
        if self.current_phase_index < self.phases.len() - 1 {
            self.current_phase_index += 1;
            self.current_phase_timer = self.phases[self.current_phase_index].phase_timer()
        }
    }
}

#[derive(Clone)]
pub struct Phase {
    state: State,
    time: Option<f32>,
}

impl Phase {
    fn for_seconds(state: State, seconds: f32) -> Self {
        Phase {
            state,
            time: Some(seconds),
        }
    }

    fn infinite(state: State) -> Self {
        Phase {
            state,
            time: None,
        }
    }

    fn phase_timer(&self) -> Option<Timer> {
        Some(Timer::from_seconds(self.time?, TimerMode::Once))
    }
}