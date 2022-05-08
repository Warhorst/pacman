use std::ops::RangeInclusive;
use bevy::prelude::*;
use crate::constants::{GHOST_SPEED, PACMAN_SPEED};
use crate::ghosts::Ghost;
use crate::ghosts::state::State;
use crate::ghosts::state::State::Frightened;
use crate::level::Level;
use crate::pacman::Pacman;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpeedByLevel::new())
            .add_system(update_ghost_speed_when_state_changed)
            .add_system(update_ghost_speed_when_state_changed)
        ;
    }
}

/// The current speed of a moving entity
#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct Speed(pub f32);

pub struct SpeedByLevel {
    pacman_ranges: Vec<LevelRangeSpeed>,
    ghost_ranges: Vec<LevelRangeSpeed>,
}

impl SpeedByLevel {
    pub fn new() -> Self {
        let pacman_ranges = vec![
            LevelRangeSpeed::new(Level(1)..=Level(1), Speed(0.8 * PACMAN_SPEED)),
            LevelRangeSpeed::new(Level(2)..=Level(4), Speed(0.9 * PACMAN_SPEED)),
            LevelRangeSpeed::new(Level(5)..=Level(20), Speed(1.0 * PACMAN_SPEED)),
            LevelRangeSpeed::new(Level(21)..=Level(65000), Speed(0.9 * PACMAN_SPEED)),
        ];

        let ghost_ranges = vec![
            LevelRangeSpeed::new(Level(1)..=Level(1), Speed(0.75 * GHOST_SPEED)),
            LevelRangeSpeed::new(Level(2)..=Level(4), Speed(0.85 * GHOST_SPEED)),
            LevelRangeSpeed::new(Level(5)..=Level(20), Speed(0.95 * GHOST_SPEED)),
            LevelRangeSpeed::new(Level(21)..=Level(65000), Speed(0.95 * GHOST_SPEED)),
        ];

        SpeedByLevel {
            pacman_ranges,
            ghost_ranges,
        }
    }

    pub fn get_pacman_speed_by_level(&self, level: &Level) -> Speed {
        Self::get_speed_for_level(self.pacman_ranges.iter(), level)
    }

    pub fn get_ghost_speed_by_level(&self, level: &Level) -> Speed {
        Self::get_speed_for_level(self.ghost_ranges.iter(), level)
    }

    fn get_speed_for_level<'a, I: IntoIterator<Item=&'a LevelRangeSpeed>>(iter: I, level: &Level) -> Speed {
        iter.into_iter()
            .find_map(|r| match r.level_in_range(level) {
                true => Some(r.speed),
                false => None
            })
            .expect("For the given level was no speed assigned")
    }
}

struct LevelRangeSpeed {
    range: RangeInclusive<Level>,
    speed: Speed,
}

impl LevelRangeSpeed {
    pub fn new(range: RangeInclusive<Level>, speed: Speed) -> Self {
        Self { range, speed }
    }

    pub fn level_in_range(&self, level: &Level) -> bool {
        self.range.contains(&level)
    }
}

// TODO: I try out change detection with this one. to keep the app consistent,
//  a "way to go" (events or change detection) should be choosen for each system.
//  Currently, it's mostly events.
fn update_ghost_speed_when_state_changed(
    mut query: Query<(&mut Speed, &State), (With<Ghost>, Changed<State>)>
) {
    for (mut speed, state) in query.iter_mut() {
        match state {
            Frightened => **speed *= 0.5,
            _ => **speed = GHOST_SPEED
        }
    }
}

fn update_pacman_speed_when_level_changed(
    speed_by_level: Res<SpeedByLevel>,
    level: Res<Level>,
    mut query: Query<&mut Speed, With<Pacman>>,
) {
    if !level.is_changed() { return; }

    for mut speed in query.iter_mut() {
        *speed = speed_by_level.get_pacman_speed_by_level(&level)
    }
}