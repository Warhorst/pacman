use bevy::prelude::*;
use crate::core::prelude::*;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
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

/// Spawn a fruit for the current level when a specific amount of dots
/// was eaten.
fn spawn_fruit_when_dot_limit_reached(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    eaten_dots: Res<EatenDots>,
    specs_per_level: Res<SpecsPerLevel>,
    mut event_reader: EventReader<DotWasEaten>,
    spawners: Query<&Tiles, With<FruitSpawn>>,
) {
    let num_eaten_dots = eaten_dots.get_eaten();

    for _ in event_reader.read() {
        if let 70 | 170 = num_eaten_dots {
            for tiles in &spawners {
                let fruit = specs_per_level.get_for(&level).fruit_to_spawn;
                commands.spawn((
                    Name::new("Fruit"),
                    SpriteBundle {
                        texture: get_texture_for_fruit(&fruit, &asset_server),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(FRUIT_DIMENSION)),
                            ..default()
                        },
                        transform: Transform::from_translation(tiles.to_vec3(FRUIT_Z)),
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
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<FruitWasEaten>,
) {
    for _ in event_reader.read() {
        commands.spawn((
            Name::new("FruitEatenSound"),
            SoundEffect::new(1),
            AudioBundle {
                source: asset_server.load("sounds/fruit_eaten.ogg"),
                ..default()
            }
        ));
    }
}