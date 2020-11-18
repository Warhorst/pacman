use std::ops::DerefMut;

use bevy::prelude::*;

use crate::common::Position;
use crate::common::Direction::*;
use crate::common::Movement;
use crate::common::Movement::*;
use crate::constants::PACMAN_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType::*;
use crate::pacman::mover::Mover;

mod mover;

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(move_pacman.system())
            .add_system(walk_through_tunnel.system());
    }
}

pub struct Pacman;

fn spawn_pacman(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let start_position = board.position_of_type(PacManSpawn).clone();
    let pacman_dimension = Vec2::new(PACMAN_DIMENSION, PACMAN_DIMENSION);
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(board.coordinates_of_position(&start_position)),
            sprite: Sprite::new(pacman_dimension),
            ..Default::default()
        })
        .with(Pacman)
        .with(Idle)
        .with(start_position);
}

/// System for moving pacman around the map.
///
/// Pacman tries to move in the direction he is currently heading. If the next position
/// is an obstacle, his movement might get limited once he reached it.
/// Pacman will not move if he is currently Idle.
fn move_pacman(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, board: Res<Board>, mut query: Query<With<Pacman, (&mut Movement, &mut Position, &mut Transform)>>) {
    for (mut movement, mut position, mut transform) in query.iter_mut() {
        set_direction(&keyboard_input, &mut movement);
        let mut mover = Mover::new(&board, time.delta_seconds, movement.deref_mut(), position.deref_mut(), &mut transform.translation);
        mover.move_pacman()
    }
}

fn set_direction(keyboard_input: &Input<KeyCode>, movement: &mut Movement) {
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

fn walk_through_tunnel(board: Res<Board>, mut query: Query<With<Pacman, (&Movement, &mut Position, &mut Transform)>>) {
    for (movement, mut position, mut transform) in query.iter_mut() {
        let direction = match movement {
            Idle => return,
            Moving(dir) => dir
        };

        match direction {
            Up | Down => return,
            Right => walk_through_right_tunnel(&board, &mut position, &mut transform.translation),
            Left => walk_through_left_tunnel(&board, &mut position, &mut transform.translation)
        }
    }
}

fn walk_through_right_tunnel(board: &Board, position: &mut Position, translation: &mut Vec3) {
    let right_tunnel_position = board.position_of_type(RightTunnel);
    let left_tunnel_position = board.position_of_type(LeftTunnel);
    match position == right_tunnel_position {
        false => return,
        true => {
            *translation = board.coordinates_of_position(left_tunnel_position);
            *position = *left_tunnel_position;
        }
    }
}

fn walk_through_left_tunnel(board: &Board, position: &mut Position, translation: &mut Vec3) {
    let right_tunnel_position = board.position_of_type(RightTunnel);
    let left_tunnel_position = board.position_of_type(LeftTunnel);
    match position == left_tunnel_position {
        false => return,
        true => {
            *translation = board.coordinates_of_position(right_tunnel_position);
            *position = *right_tunnel_position;
        }
    }
}