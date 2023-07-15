use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use bevy::ui::PositionType::Absolute;
use crate::game::edibles::fruit::Fruit;
use crate::game::edibles::fruit::Fruit::*;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::level::Level;
use crate::game_state::GameState::{GameOver, InGame, Start};
use crate::game::lives::Lives;
use crate::game::specs_per_level::SpecsPerLevel;

pub(in crate::ui) struct BottomUIPlugin;

impl Plugin for BottomUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Start), spawn_bottom_ui)
            // TODO dont run only in state InGame
            .add_systems(Update, (
                update_lives,
                update_fruits
            ).run_if(in_state(InGame)))
            .add_systems(OnExit(GameOver), despawn_bottom_ui)
        ;
    }
}

#[derive(Component)]
struct BottomUI;

#[derive(Component)]
struct UILives;

#[derive(Component)]
struct UILive;

#[derive(Component)]
struct UIFruits;

#[derive(Component)]
struct UIFruit;

fn spawn_bottom_ui(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    lives: Res<Lives>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    let bottom_ui = commands.spawn((
        Name::new("BottomUI"),
        BottomUI,
        NodeBundle {
            style: Style {
                width: Percent(40.0),
                height: Percent(10.0),
                justify_content: JustifyContent::SpaceBetween,
                top: Percent(90.0),
                left: Percent(30.0),
                position_type: Absolute,
                ..default()
            },
            ..default()
        }
    )).id();

    let ui_lives = spawn_ui_lives(&mut commands, &loaded_assets, &lives);
    let ui_fruits = spawn_ui_fruits(&mut commands, &loaded_assets, &level, &specs_per_level);

    commands.entity(bottom_ui).push_children(&[ui_lives, ui_fruits]);
}

fn spawn_ui_lives(
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
    lives: &Lives,
) -> Entity {
    let ui_lives = commands.spawn((
        Name::new("UILives"),
        UILives,
        NodeBundle {
            style: Style {
                width: Percent(40.0),
                height: Percent(50.0),
                position_type: Absolute,
                bottom: Percent(40.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }
    )).id();

    let ui_live_vec = (0..**lives).into_iter()
        .map(|i| spawn_ui_live(i, commands, loaded_assets))
        .collect::<Vec<_>>();

    commands.entity(ui_lives).push_children(&ui_live_vec);
    ui_lives
}

fn spawn_ui_live(
    index: usize,
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
) -> Entity {
    let image = loaded_assets.get_handle("textures/pacman/pacman_life.png");
    commands.spawn((
        Name::new("UILive"),
        UILive,
        ImageBundle {
            image: UiImage::new(image.clone()),
            style: Style {
                width: Percent(20.0),
                height: Percent(100.0),
                left: Percent(index as f32 * 20.0),
                position_type: Absolute,
                ..default()
            },
            ..default()
        },
    )).id()
}

fn spawn_ui_fruits(
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) -> Entity {
    let ui_fruits = commands.spawn((
        Name::new("UIFruits"),
        UIFruits,
        NodeBundle {
            style: Style {
                width: Percent(60.0),
                height: Percent(50.0),
                position_type: Absolute,
                left: Percent(40.0),
                bottom: Percent(40.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }
    )).id();

    let fruits_to_display = get_fruits_to_display(&level, &specs_per_level);

    for (i, fruit) in fruits_to_display.into_iter().enumerate() {
        let ui_fruit = spawn_ui_fruit(commands, loaded_assets, i, fruit);
        commands.entity(ui_fruits).push_children(&[ui_fruit]);
    }

    ui_fruits
}

fn get_fruits_to_display(
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) -> Vec<Fruit> {
    let border = level.checked_sub(6).unwrap_or(1).max(1);
    (border..=**level).rev()
        .into_iter()
        .map(|i| specs_per_level.get_for(&Level(i)).fruit_to_spawn)
        .collect()
}

fn spawn_ui_fruit(
    commands: &mut Commands,
    loaded_assets: &LoadedAssets,
    index: usize,
    fruit: Fruit,
) -> Entity {
    let image = get_texture_for_fruit(&fruit, loaded_assets);
    let left_percent = 100.0 - index as f32 * (100.0 / 7.0) - 100.0 / 7.0;

    commands.spawn((
        Name::new("UIFruit"),
        UIFruit,
        ImageBundle {
            image: UiImage::new(image),
            style: Style {
                width: Percent(100.0 / 7.0),
                height: Percent(100.0),
                left: Percent(left_percent),
                position_type: Absolute,
                ..default()
            },
            ..default()
        }
    )).id()
}

fn get_texture_for_fruit(fruit: &Fruit, loaded_assets: &LoadedAssets) -> Handle<Image> {
    loaded_assets.get_handle(&format!("textures/fruits/{}.png", match fruit {
        Cherry => "cherry",
        Strawberry => "strawberry",
        Peach => "peach",
        Apple => "apple",
        Grapes => "grapes",
        Galaxian => "galaxian",
        Bell => "bell",
        Key => "key"
    }))
}

/// Update the lives ui by despawning it and respawn it with the updated amount of lives.
fn update_lives(
    mut commands: Commands,
    lives: Res<Lives>,
    loaded_assets: Res<LoadedAssets>,
    bottom_ui_query: Query<Entity, With<BottomUI>>,
    ui_lives_query: Query<Entity, With<UILives>>,
) {
    if lives.is_changed() {
        for e in &ui_lives_query {
            commands.entity(e).despawn_recursive();
        }

        for bottom_ui in &bottom_ui_query {
            let ui_lives = spawn_ui_lives(&mut commands, &loaded_assets, &lives);
            commands.entity(bottom_ui).push_children(&[ui_lives]);
        }
    }
}

fn update_fruits(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    bottom_ui_query: Query<Entity, With<BottomUI>>,
    ui_fruits_query: Query<Entity, With<UIFruits>>,
) {
    if level.is_changed() {
        for e in &ui_fruits_query {
            commands.entity(e).despawn_recursive();
        }

        for bottom_ui in &bottom_ui_query {
            let ui_fruits = spawn_ui_fruits(&mut commands, &loaded_assets, &level, &specs_per_level);
            commands.entity(bottom_ui).push_children(&[ui_fruits]);
        }
    }
}

fn despawn_bottom_ui(
    mut commands: Commands,
    query: Query<Entity, With<BottomUI>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}