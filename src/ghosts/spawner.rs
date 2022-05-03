use bevy::prelude::*;

use crate::common::{Movement, Position};
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::Ghost;
use crate::ghosts::Ghost::*;
use crate::ghosts::movement::MovementReverseMarker;
use crate::ghosts::state::State;
use crate::ghosts::target::Target;
use crate::map::board::Board;
use crate::map::FieldType;

pub(in crate::ghosts) struct Spawner<'a> {
    commands: Commands<'a, 'a>,
    board: &'a Board,
}

impl<'a> Spawner<'a> {
    pub fn new(commands: Commands<'a, 'a>, board: &'a Board) -> Self {
        Spawner { commands, board }
    }

    pub fn spawn(&mut self) {
        let spawn_positions = self.board.positions_of_type(FieldType::GhostSpawn);
        self.spawn_ghost(spawn_positions[0], Blinky);
        self.spawn_ghost(spawn_positions[1], Pinky);
        self.spawn_ghost(spawn_positions[2], Inky);
        self.spawn_ghost(spawn_positions[3], Clyde)
    }

    fn spawn_ghost(&mut self, position: &Position, ghost: Ghost) {
        let color = match ghost {
            Blinky => Color::hex("FF0000").unwrap(),
            Pinky => Color::hex("FFB8FF").unwrap(),
            Inky => Color::hex("00FFFF").unwrap(),
            Clyde => Color::hex("FFB852").unwrap(),
        };
        self.commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
                    ..default()
                },
                transform: Transform::from_translation(self.board.coordinates_of_position(position)),
                ..Default::default()
            })
            .insert(ghost)
            .insert(*position)
            .insert(Target::new())
            .insert(Movement::Idle)
            .insert(State::Spawned)
            .insert(MovementReverseMarker::new());
    }
}