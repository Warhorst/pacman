use bevy::prelude::*;
use crate::constants::FONT;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::life_cycle::LifeCycle::GameOver;
use crate::map::FruitSpawn;

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
    game_asset_handles: Res<LoadedAssets>,
    fruit_spawn_query: Query<&FruitSpawn>,
) {
    let transform = Transform::from_translation(fruit_spawn_query.single().0);
    commands.spawn((
        Name::new("GameOverScreen"),
        GameOverScreen,
        Text2dBundle {
            text: Text::from_section(
                "GAME OVER".to_string(),
                TextStyle {
                    font: game_asset_handles.get_handle(FONT),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 0.0, 0.0),
                },
            ).with_alignment(
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform,
            ..Default::default()
        }
    ));
}