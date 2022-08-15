use std::time::Duration;
use bevy::prelude::*;

use crate::constants::{ENERGIZER_DIMENSION, ENERGIZER_Z};
use crate::interactions::EEnergizerEaten;
use crate::life_cycle::LifeCycle::*;
use crate::is;
use crate::level::Level;
use crate::map::Element::EnergizerSpawn;
use crate::map::Map;

pub struct EnergizerPlugin;

impl Plugin for EnergizerPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_event::<EnergizerOver>()
            .add_system_set(
                SystemSet::on_enter(Start).with_system(spawn_energizer)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(start_energizer_timer_when_energizer_eaten)
                    .with_system(update_energizer_timer.after(start_energizer_timer_when_energizer_eaten))
            )
        ;
    }
}

/// An energizer that allows pacman to eat ghosts.
#[derive(Component)]
pub struct Energizer;

/// Fired when an energizer is no longer active
pub struct EnergizerOver;

pub struct EnergizerTimer {
    timer: Timer,
}

impl EnergizerTimer {
    /// The energizer is active for the full time at level 1.
    /// Its time gets reduced every level until level 19, were it stops instantly.
    ///
    /// I use a linear function to calculate the energizer time per level. This is only speculation.
    /// It is unclear how the time an energizer is active gets calculated.
    pub fn start(level: &Level) -> Self {
        let level = **level as f32 - 1.0;
        let time = f32::max(8.0 - level * (8.0 / 18.0), 0.0);

        EnergizerTimer {
            timer: Timer::from_seconds(time, false)
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }

    /// Return the remaining seconds for this timer (if the timer is active, else None)
    pub fn remaining(&self) -> f32 {
        self.timer.duration().as_secs_f32() - self.timer.elapsed_secs()
    }
}

fn spawn_energizer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>
) {
    let energizer_dimension = Vec2::new(ENERGIZER_DIMENSION, ENERGIZER_DIMENSION);
    for position in map.get_positions_matching(is!(EnergizerSpawn)) {
        let mut coordinates = Vec3::from(position);
        coordinates.z = ENERGIZER_Z;

        commands.spawn()
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/energizer.png"),
                sprite: Sprite {
                    custom_size: Some(energizer_dimension),
                    ..default()
                },
                transform: Transform::from_translation(coordinates),
                ..Default::default()
            })
            .insert(Energizer)
            .insert(Energizer)
        ;
    }
}

fn start_energizer_timer_when_energizer_eaten(
    mut commands: Commands,
    mut event_reader: EventReader<EEnergizerEaten>,
    level: Res<Level>,
) {
    for _ in event_reader.iter() {
        commands.insert_resource(EnergizerTimer::start(&level));
    }
}

fn update_energizer_timer(
    mut commands: Commands,
    mut event_writer: EventWriter<EnergizerOver>,
    energizer_timer: Option<ResMut<EnergizerTimer>>,
    time: Res<Time>
) {
    if let Some(mut timer) = energizer_timer {
        timer.tick(time.delta());

        if timer.is_finished() {
            commands.remove_resource::<EnergizerTimer>();
            event_writer.send(EnergizerOver);
        }
    }


}