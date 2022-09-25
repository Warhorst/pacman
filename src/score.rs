use std::time::Duration;
use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;

use crate::constants::{POINTS_PER_DOT, POINTS_PER_ENERGIZER, POINTS_PER_GHOST, TEXT_Z};
use crate::edibles::energizer::EnergizerOver;
use crate::interactions::{EDotEaten, EEnergizerEaten, EFruitEaten, EPacmanEatsGhost};
use crate::life_cycle::LifeCycle::{PacmanHit, Running, Start};
use crate::edibles::fruit::Fruit::*;
use crate::game_assets::loaded_assets::LoadedAssets;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .insert_resource(EatenGhostCounter(0))
            .add_system_set(
                SystemSet::on_enter(Start).with_system(create_score_boards)
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(update_scoreboard)
                    .with_system(add_points_for_eaten_dot)
                    .with_system(add_points_for_eaten_energizer)
                    .with_system(add_points_for_eaten_ghost_and_display_score_text)
                    .with_system(reset_eaten_ghost_counter_when_energizer_is_over)
                    .with_system(add_points_for_eaten_fruit_and_display_score_text)
                    .with_system(update_score_texts)
            )
            .add_system_set(
                SystemSet::on_enter(PacmanHit).with_system(despawn_score_texts)
            )
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
pub struct ScoreBoard;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Deref, DerefMut)]
pub struct ScoreTextTimer(Timer);

#[derive(Deref, DerefMut)]
struct EatenGhostCounter(usize);

fn create_score_boards(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
    dimensions: Res<BoardDimensions>
) {
    let origin = dimensions.origin();

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "0".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            }
        ),
        transform: Transform::from_xyz(origin.x, origin.y + dimensions.board_height(), 0.0),
        ..Default::default()
    })
        .insert(ScoreBoard);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "0".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        ),
        transform: Transform::from_xyz(origin.x + dimensions.board_width() / 2.0, origin.y + dimensions.board_height(), 0.0),
        ..Default::default()
    })
        .insert(ScoreBoard);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "HIGH SCORE".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        ),
        transform: Transform::from_xyz(origin.x + dimensions.board_width() / 2.0, origin.y + dimensions.board_height() + dimensions.field(), 0.0),
        ..Default::default()
    });
}

fn update_scoreboard(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreBoard>>,
) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", **score)
    }
}

fn add_points_for_eaten_dot(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<EDotEaten>,
) {
    for _ in event_reader.iter() {
        score.add(POINTS_PER_DOT)
    }
}

fn add_points_for_eaten_energizer(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<EEnergizerEaten>,
) {
    for _ in event_reader.iter() {
        score.add(POINTS_PER_ENERGIZER)
    }
}

fn add_points_for_eaten_ghost_and_display_score_text(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
    mut score: ResMut<Score>,
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
    mut event_reader: EventReader<EPacmanEatsGhost>,
) {
    for event in event_reader.iter() {
        let points = POINTS_PER_GHOST * 2usize.pow(**eaten_ghost_counter as u32);
        score.add(points);
        **eaten_ghost_counter += 1;

        let mut coordinates = event.1.translation;
        coordinates.z = TEXT_Z;
        spawn_score_text(&mut commands, &game_asset_handles, Color::hex("31FFFF").unwrap(), points, coordinates)
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

fn add_points_for_eaten_fruit_and_display_score_text(
    mut commands: Commands,
    game_asset_handles: Res<LoadedAssets>,
    mut score: ResMut<Score>,
    mut event_reader: EventReader<EFruitEaten>,
) {
    for event in event_reader.iter() {
        let (fruit, transform) = (event.0, event.1);

        let points = match fruit {
            Cherry => 100,
            Strawberry => 300,
            Peach => 500,
            Apple => 700,
            Grapes => 1000,
            Galaxian => 2000,
            Bell => 3000,
            Key => 5000
        };

        let mut coordinates = transform.translation;
        coordinates.z = TEXT_Z;

        score.add(points);
        spawn_score_text(&mut commands, &game_asset_handles, Color::hex("FFBDFF").unwrap(), points, coordinates)
    }
}

fn spawn_score_text(
    commands: &mut Commands,
    game_asset_handles: &LoadedAssets,
    color: Color,
    points: usize,
    coordinates: Vec3
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            points.to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color,
            },
        ).with_alignment(
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }
        ),
        transform: Transform::from_translation(coordinates),
        ..Default::default()
    })
        .insert(ScoreText)
        .insert(ScoreTextTimer(Timer::new(Duration::from_secs(1), false)))
    ;
}

fn update_score_texts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScoreTextTimer), With<ScoreText>>
) {
    for (entity, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_score_texts(
    mut commands: Commands,
    query: Query<Entity, With<ScoreText>>
) {
    for e in &query {
        commands.entity(e).despawn()
    }
}