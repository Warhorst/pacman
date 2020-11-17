use bevy::prelude::*;

use Ghost::*;
use State::*;

use crate::common::Position;
use crate::constants::GHOST_DIMENSION;
use crate::map::board::Board;
use crate::map::FieldType;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Target(Option<Position>);

/// The different states of a ghost#
///
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
enum State {
    Chase,
    Scatter,
    Eaten,
    Frightened,
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ghosts.system())
            .add_system(set_target.system());

    }
}

fn spawn_ghosts(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let spawn_positions = board.positions_of_type(FieldType::GhostSpawn);
    spawn_ghost(spawn_positions[0], Blinky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[1], Pinky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[2], Inky, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[3], Clyde, &mut commands, &board, &mut materials)
}

fn spawn_ghost(position: &Position, ghost: Ghost, commands: &mut Commands, board: &Res<Board>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let color_material = match ghost {
        Blinky => Color::hex("FF0000").unwrap().into(),
        Pinky => Color::hex("FFB8FF").unwrap().into(),
        Inky => Color::hex("00FFFF").unwrap().into(),
        Clyde => Color::hex("FFB852").unwrap().into(),
    };
    commands
        .spawn(SpriteComponents {
            material: materials.add(color_material),
            transform: Transform::from_translation(board.coordinates_of_position(position)),
            sprite: Sprite::new(Vec2::new(GHOST_DIMENSION, GHOST_DIMENSION)),
            ..Default::default()
        })
        .with(ghost)
        .with(*position)
        .with(Target(None))
        .with(Scatter);
}

/// Set the ghosts target if he does not have one.
fn set_target(board: Res<Board>, mut query: Query<(&Ghost, &Position, &mut Target, &State)>) {
    for (ghost, position, mut target, state) in query.iter_mut() {
        if target.0.is_some() {
            continue
        }

        target.0 = match state {
            Scatter => Some(get_scatter_target(&board, *ghost, position)),
            _ => unimplemented!()
        }
    }
}

fn get_scatter_target(board: &Board, ghost: Ghost, position: &Position) -> Position {
    println!("Set target of {:?}", ghost);
    *position
}