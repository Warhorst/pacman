use std::time::Duration;
use bevy::prelude::*;
use crate::game::level::Level;
use Fruit::*;
use crate::constants::FRUIT_DIMENSION;
use crate::game::edibles::dots::EatenDots;
use crate::game::edibles::Edible;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::game::interactions::{DotWasEaten, FruitWasEaten};
use crate::game_state::GameState::*;
use crate::game_state::Game::*;
use crate::game::map::FruitSpawn;
use crate::game::specs_per_level::SpecsPerLevel;
use crate::system_sets::ProcessIntersectionsWithPacman;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<FruitEatenSound>()
            .add_systems(
                Update,
                (
                    spawn_fruit_when_dot_limit_reached
                        .in_set(ProcessIntersectionsWithPacman),
                    update_despawn_timer,
                    despawn_fruit_if_timer_exceeded,
                    play_fruit_eaten_sound_when_fruit_was_eaten
                        .in_set(ProcessIntersectionsWithPacman),
                    reset_fruit_despawn_timer_when_level_changed
                )
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                Update,
                update_fruit_eaten_sound_timer
            )
            .add_systems(
                OnEnter(Game(PacmanHit)),
                despawn_fruit_and_timer
            )
            .add_systems(
                OnEnter(Game(LevelTransition)),
                despawn_fruit_and_timer
            )
            .add_systems(
                OnExit(Game(GameOver)),
                despawn_fruit_and_timer
            )
        ;
    }
}

#[derive(Copy, Clone, Component, Debug, Default, Eq, PartialEq)]
pub enum Fruit {
    #[default]
    Cherry,
    Strawberry,
    Peach,
    Apple,
    Grapes,
    Galaxian,
    Bell,
    Key,
}

#[derive(Deref, DerefMut, Resource)]
pub struct FruitDespawnTimer(Timer);

impl FruitDespawnTimer {
    fn new() -> Self {
        FruitDespawnTimer(Timer::new(Duration::from_secs_f32(9.5), TimerMode::Once))
    }
}

/// The sound that plays when a fruit was eaten. Has a timer to it
/// to check if it can be despawned.
#[derive(Component, Reflect)]
struct FruitEatenSound {
    timer: Timer
}

impl FruitEatenSound {
    fn new() -> Self {
        FruitEatenSound {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }

    fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    fn finished(&self) -> bool {
        self.timer.finished()
    }
}

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>,
    mut event_reader: EventReader<DotWasEaten>,
    spawn_query: Query<&FruitSpawn>,
) {
    let num_eaten_dots = eaten_dots.get_eaten();

    for _ in event_reader.iter() {
        if let 70 | 170 = num_eaten_dots {
            for spawn in &spawn_query {
                let fruit = specs_per_level.get_for(&level).fruit_to_spawn;
                commands.spawn((
                    Name::new("Fruit"),
                    SpriteBundle {
                        texture: get_texture_for_fruit(&fruit, &loaded_assets),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(FRUIT_DIMENSION)),
                            ..default()
                        },
                        transform: Transform::from_translation(**spawn),
                        ..Default::default()
                    },
                    fruit,
                    Edible
                ));
            }
            commands.insert_resource(FruitDespawnTimer::new());
        }
    }
}

/// Update the despawn timer with delta time.
fn update_despawn_timer(
    time: Res<Time>,
    mut despawn_timer_opt: Option<ResMut<FruitDespawnTimer>>,
) {
    if let Some(ref mut despawn_timer) = despawn_timer_opt {
        despawn_timer.tick(time.delta());
    }
}

/// When the fruit despawn timer exceeds, remove the fruit and the timer resource.
fn despawn_fruit_if_timer_exceeded(
    mut commands: Commands,
    despawn_timer_opt: Option<Res<FruitDespawnTimer>>,
    query: Query<Entity, With<Fruit>>,
) {
    if let Some(ref despawn_timer) = despawn_timer_opt {
        if despawn_timer.finished() {
            for entity in query.iter() {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>()
            }
        }
    }
}

/// If the level changed, remove the timer and reset the dot counter.
fn reset_fruit_despawn_timer_when_level_changed(
    mut commands: Commands,
    level: Res<Level>,
) {
    if level.is_changed() {
        commands.remove_resource::<FruitDespawnTimer>();
    }
}

fn despawn_fruit_and_timer(
    mut commands: Commands,
    query: Query<Entity, With<Fruit>>,
) {
    commands.remove_resource::<FruitDespawnTimer>();

    for e in &query {
        commands.entity(e).despawn()
    }
}

fn play_fruit_eaten_sound_when_fruit_was_eaten(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    mut event_reader: EventReader<FruitWasEaten>,
) {
    for _ in event_reader.iter() {
        commands.spawn((
            Name::new("FruitEatenSound"),
            FruitEatenSound::new(),
            AudioBundle {
                source: loaded_assets.get_handle("sounds/fruit_eaten.ogg"),
                ..default()
            }
        ));
    }
}

fn get_texture_for_fruit(fruit: &Fruit, asset_handles: &LoadedAssets) -> Handle<Image> {
    asset_handles.get_handle(&format!("textures/fruits/{}.png", match fruit {
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

/// Updates the timer on a fruit eaten sound. As I currently know no other way to check if a sound
/// finished playing, this is the solution.
fn update_fruit_eaten_sound_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut sounds: Query<(Entity, &mut FruitEatenSound)>
) {
    let delta = time.delta();

    for (entity, mut sound) in &mut sounds {
        sound.update(delta);

        if sound.finished() {
            commands.entity(entity).despawn();
        }
    }
}