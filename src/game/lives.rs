use bevy::prelude::*;
use crate::core::prelude::*;

pub(super) struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app
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

fn remove_life_when_pacman_dies(
    mut event_reader: EventReader<PacmanWasHit>,
    mut lives: ResMut<Lives>,
) {
    if event_reader.read().count() > 0 && **lives > 0 {
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

