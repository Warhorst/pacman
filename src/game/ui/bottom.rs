use bevy::prelude::*;
use bevy::prelude::PositionType::Absolute;
use bevy::prelude::Val::Percent;
use crate::core::prelude::*;

pub(super) struct BottomUIPlugin;

impl Plugin for BottomUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Game(Start)),
                spawn_bottom_ui,
            )
            .add_systems(
                Update,
                (
                    update_lives,
                    update_fruits
                ).run_if(in_game))
            .add_systems(
                OnExit(Game(GameOver)),
                despawn_bottom_ui
            )
        ;
    }
}

/// Parent component for the ui beneath the maze
#[derive(Component)]
struct BottomUI;

/// Parent component for all ui lives. For organization purposes only.
#[derive(Component)]
struct UILives;

/// Component which represents a ui live. These are used to visualize the lives the player has left.
#[derive(Component)]
struct UILive;

/// Parent component for all ui fruites. For organization purposes only.
#[derive(Component)]
struct UIFruits;

/// Parent component for a ui fruit. These are used to show the player which fruit to expect and what past fruits occurred.
#[derive(Component)]
struct UIFruit;

fn spawn_bottom_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lives: Res<Lives>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
) {
    let bottom_ui = commands.spawn((
        Name::new("BottomUI"),
        BottomUI,
        Node {
            width: Percent(40.0),
            height: Percent(10.0),
            justify_content: JustifyContent::SpaceBetween,
            top: Percent(90.0),
            left: Percent(30.0),
            position_type: Absolute,
            ..default()
        },
    )).id();

    let ui_lives = spawn_ui_lives(&mut commands, &asset_server, &lives);
    let ui_fruits = spawn_ui_fruits(&mut commands, &asset_server, &level, &specs_per_level);

    commands.entity(bottom_ui).add_children(&[ui_lives, ui_fruits]);
}

fn spawn_ui_lives(
    commands: &mut Commands,
    asset_server: &AssetServer,
    lives: &Lives,
) -> Entity {
    let ui_lives = commands.spawn((
        Name::new("UILives"),
        UILives,
        Node {
            width: Percent(40.0),
            height: Percent(50.0),
            position_type: Absolute,
            bottom: Percent(40.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).id();

    let ui_live_vec = (0..**lives).into_iter()
        .map(|i| spawn_ui_live(i, commands, asset_server))
        .collect::<Vec<_>>();

    commands.entity(ui_lives).add_children(&ui_live_vec);
    ui_lives
}

fn spawn_ui_live(
    index: usize,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    let image = asset_server.load("textures/pacman/pacman_life.png");
    commands.spawn((
        Name::new("UILive"),
        UILive,
        Node {
            width: Percent(20.0),
            height: Percent(100.0),
            left: Percent(index as f32 * 20.0),
            position_type: Absolute,
            ..default()
        },
        ImageNode::new(image.clone()),
    )).id()
}

fn spawn_ui_fruits(
    commands: &mut Commands,
    asset_server: &AssetServer,
    level: &Level,
    specs_per_level: &SpecsPerLevel,
) -> Entity {
    let ui_fruits = commands.spawn((
        Name::new("UIFruits"),
        UIFruits,
        Node {
            width: Percent(60.0),
            height: Percent(50.0),
            position_type: Absolute,
            left: Percent(40.0),
            bottom: Percent(40.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).id();

    let fruits_to_display = get_fruits_to_display(&level, &specs_per_level);

    for (i, fruit) in fruits_to_display.into_iter().enumerate() {
        let ui_fruit = spawn_ui_fruit(commands, asset_server, i, fruit);
        commands.entity(ui_fruits).add_children(&[ui_fruit]);
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
    asset_server: &AssetServer,
    index: usize,
    fruit: Fruit,
) -> Entity {
    let image = get_texture_for_fruit(&fruit, asset_server);
    let left_percent = 100.0 - index as f32 * (100.0 / 7.0) - 100.0 / 7.0;

    commands.spawn((
        Name::new("UIFruit"),
        UIFruit,
        Node {
            width: Percent(100.0 / 7.0),
            height: Percent(100.0),
            left: Percent(left_percent),
            position_type: Absolute,
            ..default()
        },
        ImageNode::new(image),
    )).id()
}

/// Update the lives ui by despawning it and respawn it with the updated amount of lives.
fn update_lives(
    mut commands: Commands,
    lives: Res<Lives>,
    asset_server: Res<AssetServer>,
    bottom_ui_query: Query<Entity, With<BottomUI>>,
    ui_lives_query: Query<Entity, With<UILives>>,
) {
    if lives.is_changed() {
        for e in &ui_lives_query {
            commands.entity(e).despawn_recursive();
        }

        for bottom_ui in &bottom_ui_query {
            let ui_lives = spawn_ui_lives(&mut commands, &asset_server, &lives);
            commands.entity(bottom_ui).add_children(&[ui_lives]);
        }
    }
}

fn update_fruits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
            let ui_fruits = spawn_ui_fruits(&mut commands, &asset_server, &level, &specs_per_level);
            commands.entity(bottom_ui).add_children(&[ui_fruits]);
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