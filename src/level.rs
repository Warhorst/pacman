use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CurrentLevel::new());
    }
}

/// A level of a pacman game.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Level {
    value: usize
}

impl Level {
    pub fn new(value: usize) -> Self {
        Level { value }
    }

    pub fn add(&mut self, increase: usize) {
        self.value += increase
    }
}

/// The current level of the game. Increases when all dots are eaten.
pub struct CurrentLevel {
    level: Level
}

impl CurrentLevel {
    pub fn new() -> Self {
        CurrentLevel {
            level: Level::new(1)
        }
    }

    pub fn increase(&mut self) {
        self.level.add(1)
    }

    pub fn get(&self) -> Level {
        self.level
    }
}

