use bevy::prelude::*;

use crate::constants::{FIELD_DIMENSION, POINTS_PER_DOT, POINTS_PER_ENERGIZER, POINTS_PER_GHOST};
use crate::dots::DotEaten;
use crate::energizer::{EnergizerEaten, EnergizerOver};
use crate::map::board::Board;
use crate::pacman::PacmanEatsGhost;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .insert_resource(EatenGhostCounter(0))
            .add_startup_system(create_scoreboard)
            .add_system(update_scoreboard)
            .add_system(add_points_for_eaten_dot)
            .add_system(add_points_for_eaten_energizer)
            .add_system(add_points_for_eaten_ghost)
            .add_system(reset_eaten_ghost_counter_when_energizer_is_over)
        ;
    }
}

/// Resource that saves how many points the player has collected so far
#[derive(Deref, DerefMut)]
pub struct Score(usize);

impl Score {
    fn add(&mut self, points: usize) {
        **self += points
    }
}

#[derive(Component)]
pub struct Scoreboard;

#[derive(Deref, DerefMut)]
struct EatenGhostCounter(usize);

fn create_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Score".to_string(),
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 40.0,
                                     color: Color::rgb(1.0, 1.0, 1.0),
                                 },
                                 TextAlignment {
                                     vertical: VerticalAlign::Center,
                                     horizontal: HorizontalAlign::Center,
                                 }),
        transform: Transform::from_xyz(0.0, FIELD_DIMENSION * board.height as f32, 0.0),
        ..Default::default()
    })
        .insert(Scoreboard);
}

fn update_scoreboard(
    score: Res<Score>,
    mut query: Query<&mut Text, With<Scoreboard>>,
) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", **score)
    }
}

fn add_points_for_eaten_dot(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<DotEaten>,
) {
    for _ in event_reader.iter() {
        score.add(POINTS_PER_DOT)
    }
}

fn add_points_for_eaten_energizer(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<EnergizerEaten>,
) {
    for _ in event_reader.iter() {
        score.add(POINTS_PER_ENERGIZER)
    }
}

fn add_points_for_eaten_ghost(
    mut score: ResMut<Score>,
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
    mut event_reader: EventReader<PacmanEatsGhost>,
) {
    for _ in event_reader.iter() {
        score.add(POINTS_PER_GHOST * 2usize.pow(**eaten_ghost_counter as u32));
        **eaten_ghost_counter += 1
    }
}

fn reset_eaten_ghost_counter_when_energizer_is_over(
    mut event_reader: EventReader<EnergizerOver>,
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
) {
    for _ in event_reader.iter() {
        **eaten_ghost_counter = 0
    }
}