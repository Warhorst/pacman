use bevy::prelude::*;
use crate::board_dimensions::BoardDimensions;
use crate::constants::TEXT_Z;
use crate::game_assets::loaded_assets::LoadedAssets;
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
    game_asset_handles: Res<LoadedAssets>,
    map: Res<Map>,
    dimensions: Res<BoardDimensions>
) {
    let transform = dimensions.positions_to_trans(map.get_positions_matching(is!(Element::FruitSpawn)), TEXT_Z);
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "GAME OVER".to_string(),
            TextStyle {
                font: game_asset_handles.get_handle("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
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
    })
        .insert(GameOverScreen);
}