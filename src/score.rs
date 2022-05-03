use bevy::prelude::*;

use crate::constants::POINTS_PER_DOT;
use crate::dots::DotEaten;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score::new())
            .add_startup_system(create_scoreboard)
            .add_system(update_scoreboard)
            .add_system(add_points_for_eaten_dot);
    }
}

pub struct Score {
    points: usize
}

impl Score {
    fn new() -> Self {
        Score {
            points: 0
        }
    }

    pub fn increment(&mut self) {
        self.points += POINTS_PER_DOT
    }

    pub fn get_points(&self) -> usize {
        self.points
    }
}

fn create_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        transform: Transform::from_xyz(-430.0, 300.0, 0.0),
        ..Default::default()
    });
}

fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.get_points())
    }
}

fn add_points_for_eaten_dot(mut score: ResMut<Score>,
                            mut event_reader: EventReader<DotEaten>) {
    for _ in event_reader.iter() {
        score.increment()
    }
}