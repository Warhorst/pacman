use bevy::prelude::*;

use crate::common::Movement;
use crate::constants::PACMAN_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType::PacManSpawn;
use crate::pacman::Pacman;

pub (in crate::pacman) struct Spawner<'a> {
    commands: &'a mut Commands,
    board: &'a Board,
    materials: &'a mut Assets<ColorMaterial>
}

impl<'a> Spawner<'a> {
    pub fn new(commands: &'a mut Commands, board: &'a Board, materials: &'a mut Assets<ColorMaterial>) -> Self {
        Spawner { commands, board, materials }
    }

    pub fn spawn(&mut self) {
        let start_position = self.board.position_of_type(PacManSpawn).clone();
        let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
        self.commands
            .spawn(SpriteBundle {
                material: self.materials.add(Color::hex("FFEE00").unwrap().into()),
                transform: Transform::from_translation(self.board.coordinates_of_position(&start_position)),
                sprite: Sprite::new(pacman_dimension),
                ..Default::default()
            })
            .with(Pacman)
            .with(Movement::Idle)
            .with(start_position);
    }
}