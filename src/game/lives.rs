use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use crate::game::interactions::EPacmanHit;
use crate::game_state::GameState::*;
use crate::game::score::Score;

pub (in crate::game) struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InspectorPlugin::<Lives>::new())
            .insert_resource(Lives(3))
            .insert_resource(PointsRequiredForExtraLife::new())
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(remove_life_when_pacman_dies)
                    .with_system(add_life_if_player_reaches_specific_score)
            )
        ;
    }
}

/// Resource that tells how many lives pacman currently has.
#[derive(Deref, DerefMut, Inspectable, Default, Resource)]
pub struct Lives(usize);

/// Keeps track how many points the player needs to get a new life for pacman.
#[derive(Deref, DerefMut, Resource)]
pub struct PointsRequiredForExtraLife(usize);

impl PointsRequiredForExtraLife {
    pub fn new() -> Self {
        PointsRequiredForExtraLife(10000)
    }

    pub fn increase_limit(&mut self) {
        **self += 10000
    }
}

fn remove_life_when_pacman_dies(
    mut event_reader: EventReader<EPacmanHit>,
    mut lives: ResMut<Lives>,
) {
    for _ in event_reader.iter() {
        if **lives > 0 {
            **lives -= 1;
        }
    }
}

fn add_life_if_player_reaches_specific_score(
    score: Res<Score>,
    mut lives: ResMut<Lives>,
    mut points_required_for_extra_life: ResMut<PointsRequiredForExtraLife>,
) {
    if **score >= **points_required_for_extra_life {
        **lives += 1;
        points_required_for_extra_life.increase_limit();
    }
}

