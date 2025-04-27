use std::time::Duration;
use bevy::prelude::*;

use crate::core::prelude::*;

pub(in crate::game) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .insert_resource(HighScore::new(10000))
            .insert_resource(EatenGhostCounter(0))
            .add_systems(
                Update,
                (
                    reset_eaten_ghost_counter_when_energizer_is_over,
                    update_score_texts,
                    add_points_for_eaten_dot
                        .in_set(ProcessIntersectionsWithPacman),
                    add_points_for_eaten_energizer
                        .in_set(ProcessIntersectionsWithPacman),
                    add_points_for_eaten_ghost_and_display_score_text
                        .in_set(ProcessIntersectionsWithPacman),
                    add_points_for_eaten_fruit_and_display_score_text
                        .in_set(ProcessIntersectionsWithPacman),
                    update_high_score,
                    play_highscore_broken_sound.after(update_high_score)
                )
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                OnEnter(Game(PacmanHit)),
                (
                    despawn_score_texts,
                    reset_ghost_eaten_counter
                ),
            )
            .add_systems(
                OnExit(Game(GameOver)),
                (
                    reset_score,
                    reset_high_score
                )
            )
            .add_systems(
                OnEnter(Game(LevelTransition)),
                reset_ghost_eaten_counter,
            )
        ;
    }
}

fn add_points_for_eaten_dot(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<DotWasEaten>,
) {
    for _ in event_reader.read() {
        score.add(POINTS_PER_DOT)
    }
}

fn add_points_for_eaten_energizer(
    mut score: ResMut<Score>,
    mut event_reader: EventReader<EnergizerWasEaten>,
) {
    for _ in event_reader.read() {
        score.add(POINTS_PER_ENERGIZER)
    }
}

fn add_points_for_eaten_ghost_and_display_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
    mut event_reader: EventReader<GhostWasEaten>,
) {
    for event in event_reader.read() {
        let points = POINTS_PER_GHOST * 2usize.pow(**eaten_ghost_counter as u32);
        score.add(points);
        **eaten_ghost_counter += 1;

        let mut coordinates = event.1.translation;
        coordinates.z = TEXT_Z;
        spawn_score_text(&mut commands, &asset_server, Color::Srgba(Srgba::hex("31FFFF").unwrap()), points, coordinates)
    }
}

fn reset_eaten_ghost_counter_when_energizer_is_over(
    mut event_reader: EventReader<EnergizerOver>,
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
) {
    for _ in event_reader.read() {
        **eaten_ghost_counter = 0
    }
}

fn reset_ghost_eaten_counter(
    mut eaten_ghost_counter: ResMut<EatenGhostCounter>,
) {
    **eaten_ghost_counter = 0
}

fn add_points_for_eaten_fruit_and_display_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut event_reader: EventReader<FruitWasEaten>,
) {
    for event in event_reader.read() {
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
        spawn_score_text(&mut commands, &asset_server, Color::Srgba(Srgba::hex("FFBDFF").unwrap()), points, coordinates)
    }
}

fn spawn_score_text(
    commands: &mut Commands,
    asset_server: &AssetServer,
    color: Color,
    points: usize,
    coordinates: Vec3,
) {
    commands.spawn((
        Text2d(points.to_string()),
        TextFont {
            font: asset_server.load(FONT),
            font_size: 10.0,
            ..default()
        },
        TextColor(color),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        Transform::from_translation(coordinates),
        Name::new("ScoreText"),
        ScoreText,
        ScoreTextTimer(Timer::new(Duration::from_secs(1), TimerMode::Once))
    ));
}

fn update_score_texts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScoreTextTimer), With<ScoreText>>,
) {
    for (entity, mut timer) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn update_high_score(
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
    mut event_writer: EventWriter<HighScoreWasBeaten>,
) {
    if !score.is_changed() {
        return;
    }

    if **score > high_score.score {
        high_score.score = **score;

        if !high_score.was_beaten {
            high_score.was_beaten = true;
            event_writer.write(HighScoreWasBeaten);
        }
    }
}

fn play_highscore_broken_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<HighScoreWasBeaten>,
) {
    for _ in event_reader.read() {
        commands.spawn((
            Name::new("HighScoreBrokenSound"),
            SoundEffect::new(3),
            AudioPlayer::<AudioSource>(asset_server.load("sounds/high_score.ogg")),
        ));
    }
}

fn despawn_score_texts(
    mut commands: Commands,
    query: Query<Entity, With<ScoreText>>,
) {
    for e in &query {
        commands.entity(e).despawn()
    }
}

fn reset_score(
    mut score: ResMut<Score>
) {
    score.0 = 0;
}

fn reset_high_score(
    mut high_score: ResMut<HighScore>
) {
    high_score.was_beaten = false
}