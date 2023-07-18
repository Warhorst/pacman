use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use crate::game::interactions::PacmanWasHit;
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::score::Score;
use crate::system_sets::ProcessIntersectionsWithPacman;

pub(in crate::game) struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Lives>()
            .add_plugins(ResourceInspectorPlugin::<Lives>::default())
            .insert_resource(Lives(3))
            .insert_resource(PointsRequiredForExtraLife::new())
            .add_systems(
                Update,
                (
                    remove_life_when_pacman_dies.in_set(ProcessIntersectionsWithPacman),
                    add_life_if_player_reaches_specific_score
                )
                    .run_if(in_state(Game(Running))))
            .add_systems(
                OnExit(Game(GameOver)),
                reset_lives,
            )
        ;
    }
}

/// Resource that tells how many lives pacman currently has.
#[derive(Deref, DerefMut, Reflect, Default, Resource)]
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
    mut event_reader: EventReader<PacmanWasHit>,
    mut lives: ResMut<Lives>,
) {
    if event_reader.iter().count() > 0 && **lives > 0 {
        **lives -= 1;
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

fn reset_lives(
    mut lives: ResMut<Lives>
) {
    lives.0 = 3;
}

