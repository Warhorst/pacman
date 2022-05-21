use bevy::prelude::*;

use crate::constants::POINTS_PER_DOT;
use crate::dots::DotEaten;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .add_startup_system(create_scoreboard)
            .add_system(update_scoreboard)
            .add_system(add_points_for_eaten_dot);
    }
}

/// Resource that saves how many points the player has collected so far
#[derive(Deref, DerefMut)]
pub struct Score(usize);

impl Score {
    pub fn increment(&mut self) {
        **self += POINTS_PER_DOT
    }
}

#[derive(Component)]
pub struct Scoreboard;

fn create_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        transform: Transform::from_xyz(0.0, 500.0, 0.0),
        ..Default::default()
    })
        .insert(Scoreboard);
}

fn update_scoreboard(
    score: Res<Score>,
    mut query: Query<&mut Text, With<Scoreboard>>
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", **score)
    }
}

fn add_points_for_eaten_dot(mut score: ResMut<Score>,
                            mut event_reader: EventReader<DotEaten>) {
    for _ in event_reader.iter() {
        score.increment()
    }
}