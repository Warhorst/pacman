use std::time::Duration;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::core::prelude::*;
use crate::core::prelude::Level;

pub(super) struct GhostSchedulePlugin;

impl Plugin for GhostSchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ScheduleByLevel>()
            .register_type::<GhostSchedule>()
        ;
    }
}

/// Provides a mapping from the current level to the schedule the ghosts should execute
#[derive(Resource, Reflect)]
pub struct ScheduleByLevel {
    level_schedule_map: HashMap<Level, GhostSchedule>,
    default_schedule: GhostSchedule,
}

impl ScheduleByLevel {
    pub fn new() -> Self {
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

    fn level_one() -> GhostSchedule {
        GhostSchedule::new([
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

    fn level_two_to_four() -> GhostSchedule {
        GhostSchedule::new([
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

    fn level_five_and_beyond() -> GhostSchedule {
        GhostSchedule::new([
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

    pub fn get_schedule_for_level(&self, level: &Level) -> GhostSchedule {
        self.level_schedule_map.get(level).unwrap_or(&self.default_schedule).clone()
    }
}

/// Defines what a ghost will do (chase or retreat) based on the passed time since the level started.
#[derive(Resource, Reflect, Clone)]
pub struct GhostSchedule {
    current_phase_index: usize,
    current_phase_timer: Option<Timer>,
    phases: Vec<Phase>,
}

impl GhostSchedule {
    fn new(phases: impl IntoIterator<Item=Phase>) -> Self {
        let phases = phases.into_iter().collect::<Vec<_>>();

        GhostSchedule {
            current_phase_index: 0,
            current_phase_timer: phases.get(0).expect("at least one phase must be provided").phase_timer(),
            phases,
        }
    }

    pub fn current_state(&self) -> GhostState {
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

/// Tells which state a ghost should enter and for how long
#[derive(Reflect, Clone)]
pub struct Phase {
    state: GhostState,
    time: Option<f32>,
}

impl Phase {
    fn for_seconds(state: GhostState, seconds: f32) -> Self {
        Phase {
            state,
            time: Some(seconds),
        }
    }

    fn infinite(state: GhostState) -> Self {
        Phase {
            state,
            time: None,
        }
    }

    fn phase_timer(&self) -> Option<Timer> {
        Some(Timer::from_seconds(self.time?, TimerMode::Once))
    }
}