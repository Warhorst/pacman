use std::time::Duration;
use bevy::prelude::*;
use crate::life_cycle::LifeCycle::Running;

pub struct StopPlugin;

impl Plugin for StopPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ENoLongerStopped>()
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(update_stop)
            )
        ;
    }
}

fn update_stop(
    mut commands: Commands,
    time: Res<Time>,
    mut event_writer: EventWriter<ENoLongerStopped>,
    mut query: Query<(Entity, &mut Stop)>
) {
    let delta = time.delta();

    for (entity, mut stop) in &mut query {
        if stop.tick(delta).finished() {
            commands.entity(entity).remove::<Stop>();
            event_writer.send(ENoLongerStopped(entity))
        }
    }
}

#[derive(Component)]
pub struct Stop(Timer);

/// Fired when a stop ended.
pub struct ENoLongerStopped(pub Entity);

impl Stop {
    pub fn for_seconds(seconds: f32) -> Self {
        Stop(Timer::new(Duration::from_secs_f32(seconds), false))
    }

    fn tick(&mut self, delta: Duration) -> &Timer {
        self.0.tick(delta)
    }
}