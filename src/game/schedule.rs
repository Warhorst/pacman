use bevy::prelude::*;
use crate::prelude::*;

pub(in crate::game) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScheduleByLevel::new())
            .add_systems(OnEnter(Game(Start)), register_start_schedule)
            .add_systems(Update, (
                switch_schedule_when_level_changed,
                update_schedule
            ).run_if(in_state(Game(Running))))
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
    mut schedule: ResMut<GhostSchedule>,
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
    mut schedule: ResMut<GhostSchedule>,
) {
    if energizer_timer.is_none() {
        schedule.update(time.delta());
    }
}

