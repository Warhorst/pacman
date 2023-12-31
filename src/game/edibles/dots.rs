use std::time::Duration;
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::core::prelude::*;

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ResourceInspectorPlugin::<EatenDots>::default())
            .add_systems(
                OnEnter(Game(Start)),
                (
                    spawn_dots,
                    create_eaten_dots
                ),
            )
            .add_systems(
                Update,
                play_waka_when_dot_was_eaten
                    .in_set(ProcessIntersectionsWithPacman)
                    .run_if(in_state(Game(Running))),
            )
            .add_systems(
                OnExit(Game(LevelTransition)),
                (
                    spawn_dots,
                    reset_eaten_dots
                ),
            )
            .add_systems(
                OnExit(Game(GameOver)),
                (
                    despawn_dots,
                    reset_eaten_dots
                ),
            )
        ;
    }
}

fn spawn_dots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_query: Query<&DotSpawn>,
) {
    let dots = commands.spawn((
        Name::new("Dots"),
        Dots,
        SpatialBundle::default()
    )).id();

    for spawn in &spawn_query {
        commands.entity(dots).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/dot.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(DOT_DIMENSION)),
                        ..default()
                    },
                    transform: Transform::from_translation(**spawn),
                    ..Default::default()
                },
                Dot,
                Edible,
                Name::new("Dot")
            ));
        });
    }
}

fn create_eaten_dots(
    mut commands: Commands,
    dot_spawn_query: Query<&DotSpawn>,
) {
    let num_dots = dot_spawn_query.iter().count();
    commands.insert_resource(EatenDots::new(num_dots))
}

fn reset_eaten_dots(
    mut eaten_dots: ResMut<EatenDots>
) {
    eaten_dots.reset()
}

/// Play the famous waka waka when a dot was eaten.
///
/// This code sucks, but I have no other way to do it. The problem is: If I would
/// just play the waka every time a dot was eaten, the sound would overlap. I have no
/// information if the sound finished playing, so I use a custom timer, which is set
/// to the time of the track (0.3 seconds). Another waka can play when the timer finished.
///
/// But this leads to another problem: The waka makes a pause if another dot was eaten while
/// the timer is still active. So I cache a waka if the dot was eaten while the timer is active.
/// When the timer finishes and a waka is cached, it is instantly played and the timer gets reset.
/// (This might lead to an additional waka playing, but more waka waka = more fun)
fn play_waka_when_dot_was_eaten(
    mut commands: Commands,
    time: Res<Time>,
    mut waka_timer: Local<Option<Timer>>,
    mut cached: Local<bool>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<DotWasEaten>,
) {
    if let Some(ref mut timer) = *waka_timer {
        timer.tick(time.delta());

        if timer.finished() {
            if *cached {
                timer.reset();

                commands.spawn((
                    Name::new("WakaSound"),
                    SoundEffect::new(),
                    AudioBundle {
                        source: asset_server.load("sounds/waka.ogg"),
                        ..default()
                    }
                ));

                *cached = false;
            } else {
                *waka_timer = None
            }
        }
    }

    for _ in event_reader.read() {
        match *waka_timer {
            Some(_) => *cached = true,
            None => {
                *waka_timer = Some(Timer::new(Duration::from_secs_f32(0.3), TimerMode::Once));

                commands.spawn((
                    Name::new("WakaSound"),
                    SoundEffect::new(),
                    AudioBundle {
                        source: asset_server.load("sounds/waka.ogg"),
                        ..default()
                    }
                ));
            }
        };
    }
}

fn despawn_dots(
    mut commands: Commands,
    query: Query<Entity, With<Dots>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}