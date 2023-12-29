use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;

pub(super) struct EdiblesPlugin;

impl Plugin for EdiblesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Edible>()
            .register_type::<Dots>()
            .register_type::<Dot>()
            .register_type::<EatenDots>()
            .register_type::<Fruit>()
            .register_type::<FruitDespawnTimer>()
            .register_type::<Energizers>()
            .register_type::<Energizer>()
            .register_type::<EnergizerOver>()
            .register_type::<EnergizerTimer>()
            .add_event::<EAllEdiblesEaten>()
            .add_event::<EnergizerOver>()
        ;
    }
}

/// Component for everything in the maze that is edible.
#[derive(Component, Reflect)]
pub struct Edible;

/// Event that gets fired when all edibles are eaten (or at least gone), so the maze is empty
#[derive(Event)]
pub struct EAllEdiblesEaten;

/// Parent component for all dots (for organization only)
#[derive(Component, Reflect)]
pub struct Dots;

/// A dot which can be eaten by pacman
#[derive(Component, Reflect)]
pub struct Dot;

/// Keeps track of how many dots are already eaten by pacman
#[derive(Resource, Default, Reflect)]
pub struct EatenDots {
    max: usize,
    eaten: usize,
}

impl EatenDots {
    pub(crate) fn new(num_dots: usize) -> Self {
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

    pub(crate) fn reset(&mut self) {
        self.eaten = 0
    }
}

/// Fruit which can be eaten for bonus points
#[derive(Component, Reflect, Copy, Clone, Debug, Default, Eq, PartialEq)]
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

/// Timer which keeps track on when to despawn a fruit
#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct FruitDespawnTimer(Timer);

impl FruitDespawnTimer {
    pub fn new() -> Self {
        FruitDespawnTimer(Timer::new(Duration::from_secs_f32(9.5), TimerMode::Once))
    }
}

pub fn get_texture_for_fruit(fruit: &Fruit, asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(&format!("textures/fruits/{}.png", match fruit {
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

/// Parent component for all energizer (for organization only)
#[derive(Component, Reflect)]
pub struct Energizers;

/// An energizer that allows pacman to eat ghosts.
#[derive(Component, Reflect)]
pub struct Energizer;

/// Fired when an energizer is no longer active
#[derive(Event, Reflect, Copy, Clone)]
pub struct EnergizerOver;

/// Keeps track of how long an active energizer remains active
#[derive(Resource, Reflect)]
pub struct EnergizerTimer {
    timer: Timer,
}

impl EnergizerTimer {
    pub fn start(seconds: f32) -> Self {
        EnergizerTimer {
            timer: Timer::from_seconds(seconds, TimerMode::Once)
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }

    /// Return the remaining seconds for this timer (if the timer is active, else None)
    pub fn remaining(&self) -> f32 {
        self.timer.duration().as_secs_f32() - self.timer.elapsed_secs()
    }
}