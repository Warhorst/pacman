use bevy::prelude::*;

use Name::*;
use State::*;

use crate::common::Position;
use crate::constants::GHOST_DIMENSION;
use crate::map::board::Board;

pub struct Ghost {
    name: Name
}

pub enum Name {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

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
        app.add_startup_system(spawn_ghosts.system());
    }
}

fn spawn_ghosts(mut commands: Commands, board: Res<Board>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let spawn_positions = board.get_ghost_spawn_positions();
    spawn_ghost(spawn_positions[0], Ghost { name: Blinky }, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[1], Ghost { name: Pinky }, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[2], Ghost { name: Inky }, &mut commands, &board, &mut materials);
    spawn_ghost(spawn_positions[3], Ghost { name: Clyde }, &mut commands, &board, &mut materials)
}

fn spawn_ghost(position: &Position, ghost: Ghost, commands: &mut Commands, board: &Res<Board>, mut materials: &mut ResMut<Assets<ColorMaterial>>) {
    let color_material = match &ghost.name {
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
        .with(Chase);
}