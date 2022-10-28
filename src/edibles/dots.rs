use std::time::Duration;
use bevy::prelude::*;

use crate::constants::DOT_DIMENSION;
use crate::edibles::Edible;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::interactions::EDotEaten;
use crate::life_cycle::LifeCycle::*;
use crate::is;
use crate::map::{DotSpawn, Element, TileMap};

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Start)
                    .with_system(spawn_dots)
                    .with_system(spawn_eaten_dots)
            )
            .add_system_set(
                SystemSet::on_update(Running).with_system(play_waka_when_dot_was_eaten)
            )
            .add_system_set(
                SystemSet::on_exit(LevelTransition)
                    .with_system(spawn_dots)
                    .with_system(reset_eaten_dots)
            )
        ;
    }
}

fn spawn_dots(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    spawn_query: Query<&DotSpawn>
) {
    let dots = commands.spawn()
        .insert(Name::new("Dots"))
        .insert(Dots)
        .insert_bundle(SpatialBundle::default())
        .id();

    for spawn in &spawn_query {
        commands.entity(dots).with_children(|parent| {
            parent.spawn()
                .insert_bundle(SpriteBundle {
                    texture: loaded_assets.get_handle("textures/dot.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(DOT_DIMENSION)),
                        ..default()
                    },
                    transform: Transform::from_translation(**spawn),
                    ..Default::default()
                })
                .insert(Dot)
                .insert(Edible)
                .insert(Name::new("Dot"));
        });
    }
}

fn spawn_eaten_dots(
    mut commands: Commands,
    map: Res<TileMap>,
) {
    let num_dots = map.get_positions_matching(is!(Element::DotSpawn)).into_iter().count();
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
    time: Res<Time>,
    mut waka_timer: Local<Option<Timer>>,
    mut cached: Local<bool>,
    loaded_assets: Res<LoadedAssets>,
    audio: Res<Audio>,
    mut event_reader: EventReader<EDotEaten>,
) {
    if let Some(ref mut timer) = *waka_timer {
        timer.tick(time.delta());

        if timer.finished() {
            if *cached {
                timer.reset();
                audio.play(loaded_assets.get_handle("sounds/waka.ogg"));
                *cached = false;
            } else {
                *waka_timer = None
            }
        }
    }

    for _ in event_reader.iter() {
        match *waka_timer {
            Some(_) => *cached = true,
            None => {
                *waka_timer = Some(Timer::new(Duration::from_secs_f32(0.3), false));
                audio.play(loaded_assets.get_handle("sounds/waka.ogg"));
            }
        };
    }
}

/// Parent component for all dots (for organization only)
#[derive(Component)]
pub struct Dots;

#[derive(Component)]
pub struct Dot;

pub struct EatenDots {
    max: usize,
    eaten: usize,
}

impl EatenDots {
    fn new(num_dots: usize) -> Self {
        EatenDots {
            max: num_dots,
            eaten: 0,
        }
    }

    pub fn increment(&mut self) {
        self.eaten += 1
    }

    pub fn get_eaten(&self) -> usize {
        self.eaten
    }

    pub fn get_remaining(&self) -> usize {
        self.max - self.eaten
    }

    pub fn get_max(&self) -> usize {
        self.max
    }

    fn reset(&mut self) {
        self.eaten = 0
    }
}