use bevy::prelude::*;
use crate::game_asset_handles::GameAssetHandles;
use crate::game_asset_handles::keys::FONT;
use crate::is;
use crate::life_cycle::LifeCycle::GameOver;
use crate::map::Map;
use crate::map::Element;

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameOver).with_system(spawn_screen)
            )
        ;
    }
}

#[derive(Component)]
struct GameOverScreen;

fn spawn_screen(
    mut commands: Commands,
    game_asset_handles: Res<GameAssetHandles>,
    map: Res<Map>,
) {
    let coordinates = map.coordinates_between_positions_matching(is!(Element::FruitSpawn));
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "GAME OVER".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle(FONT),
                font_size: 40.0,
                color: Color::rgb(1.0, 0.0, 0.0),
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
        .insert(GameOverScreen);
}