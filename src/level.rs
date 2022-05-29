use bevy::prelude::*;
use crate::dots::AllDotsEaten;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level(1))
            .add_startup_system(spawn_level_ui)
            .add_system(increase_level_when_all_dots_eaten)
        ;
    }
}

#[derive(Deref, DerefMut, Ord, PartialOrd, Eq, PartialEq)]
pub struct Level(pub usize);

impl Level {
    fn increase(&mut self) {
        **self += 1
    }
}

#[derive(Component)]
pub struct LevelUi;

fn spawn_level_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(format!("Level: {}", **level),
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 40.0,
                                     color: Color::rgb(1.0, 1.0, 1.0),
                                 },
                                 TextAlignment {
                                     vertical: VerticalAlign::Center,
                                     horizontal: HorizontalAlign::Center,
                                 }),
        transform: Transform::from_xyz(240.0, 500.0, 0.0),
        ..Default::default()
    })
        .insert(LevelUi);
}

fn increase_level_when_all_dots_eaten(
    mut event_reader: EventReader<AllDotsEaten>,
    mut level: ResMut<Level>,
    mut query: Query<&mut Text, With<LevelUi>>,
) {
    for _ in event_reader.iter() {
        level.increase();

        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Level: {}", **level)
        }
    }
}