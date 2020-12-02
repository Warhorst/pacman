use std::ops::DerefMut;

use bevy::prelude::*;

use crate::common::{MoveComponents, Movement};
use crate::common::Direction::*;
use crate::common::Movement::*;
use crate::map::board::Board;
use crate::pacman::mover::Mover;
use crate::pacman::spawner::Spawner;

mod mover;
mod spawner;

pub struct Pacman;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(move_pacman.system())
            .add_system(set_direction.system());
    }
}

fn spawn_pacman(commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    Spawner::new(commands, &board, &mut materials).spawn()
}

fn move_pacman(time: Res<Time>,
               board: Res<Board>,
               mut query: Query<With<Pacman, MoveComponents>>) {
    for (mut transform, mut position, mut movement) in query.iter_mut() {
        Mover::new(&board,
                   time.delta_seconds,
                   movement.deref_mut(),
                   position.deref_mut(),
                   &mut transform.translation)
            .move_pacman()
    }
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>,
                 mut query: Query<With<Pacman, &mut Movement>>) {
    for mut movement in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *movement = Moving(Left)
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *movement = Moving(Right)
        }

        if keyboard_input.pressed(KeyCode::Up) {
            *movement = Moving(Up)
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *movement = Moving(Down)
        }
    }
}