use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::{Ghost, State, Target};
use crate::ghosts::Ghost::*;
use crate::map::board::Board;
use crate::map::FieldType;

pub(in crate::ghosts) struct Spawner<'a> {
    commands: &'a mut Commands,
    board: &'a Board,
    materials: &'a mut Assets<ColorMaterial>,
}

impl<'a> Spawner<'a> {
    pub fn new(commands: &'a mut Commands, board: &'a Board, materials: &'a mut Assets<ColorMaterial>) -> Self {
        Spawner { commands, board, materials }
    }

    pub fn spawn(&mut self) {
        let spawn_positions = self.board.positions_of_type(FieldType::GhostSpawn);
        self.spawn_ghost(spawn_positions[0], Blinky);
        self.spawn_ghost(spawn_positions[1], Pinky);
        self.spawn_ghost(spawn_positions[2], Inky);
        self.spawn_ghost(spawn_positions[3], Clyde)
    }

    fn spawn_ghost(&mut self, position: &Position, ghost: Ghost) {
        let color_material = match ghost {
            Blinky => Color::hex("FF0000").unwrap().into(),
            Pinky => Color::hex("FFB8FF").unwrap().into(),
            Inky => Color::hex("00FFFF").unwrap().into(),
            Clyde => Color::hex("FFB852").unwrap().into(),
        };
        self.commands
            .spawn(SpriteBundle {
                material: self.materials.add(color_material),
                transform: Transform::from_translation(self.board.coordinates_of_position(position)),
                sprite: Sprite::new(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                ..Default::default()
            })
            .with(ghost)
            .with(*position)
            .with(Target::new())
            .with(Movement::Idle)
            .with(State::Spawned);
    }
}