use bevy::prelude::*;
use crate::common::MoveDirection;

use crate::constants::{PACMAN_DIMENSION, PACMAN_SPEED};
use crate::map::board::Board;
use crate::map::FieldType::PacManSpawn;
use crate::pacman::Pacman;
use crate::speed::Speed;

pub (in crate::pacman) struct Spawner<'a> {
    commands: Commands<'a, 'a>,
    board: &'a Board,
}

impl<'a> Spawner<'a> {
    pub fn new(commands: Commands<'a, 'a>, board: &'a Board) -> Self {
        Spawner { commands, board }
    }

    pub fn spawn(&mut self) {
        let start_position = self.board.position_of_type(PacManSpawn).clone();
        let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
        self.commands.spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("FFEE00").unwrap(),
                    custom_size: Some(pacman_dimension),
                    ..default()
                },
                transform: Transform::from_translation(self.board.coordinates_of_position(&start_position)),
                ..Default::default()
            })
            .insert(Pacman)
            .insert(MoveDirection::Up)
            .insert(Speed(PACMAN_SPEED))
            .insert(start_position);
    }
}