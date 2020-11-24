use bevy::prelude::*;

use Ghost::*;
use State::*;

use crate::common::Movement;
use crate::common::Movement::*;
use crate::common::Position;
use crate::constants::GHOST_DIMENSION;
use crate::ghosts::mover::Mover;
use crate::ghosts::target_setter::TargetSetter;
use crate::map::board::Board;
use crate::map::FieldType;
use crate::map::FieldType::*;

mod target_setter;
mod mover;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Ghost {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

pub struct Target {
    target: Option<Position>
}

impl Target {
    pub fn new() -> Self {
        Target { target: None }
    }

    pub fn is_set(&self) -> bool {
        self.target.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        !self.is_set()
    }

    pub fn set_to(&mut self, position: Position) {
        self.target = Some(position)
    }

    pub fn get_position(&self) -> &Position {
        &self.target.as_ref().expect("The target should be set at this point")
    }

    pub fn clear(&mut self) {
        self.target = None
    }
}

/// The different states of a ghost
///
/// Spawned - just spawned, try to leave the spawn area
/// Chase - use your hunting strategy to kill pacman
/// Scatter - be inactive and return to your home corner
/// Eaten - return to the home to respawn
/// Frightened - you are vulnerable, dodge pacman
#[derive(Debug, PartialOrd, PartialEq)]
enum State {
    Spawned,
    // Chase,
    Scatter,
    // Eaten,
    // Frightened,
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ghosts.system())
            .add_system(set_target.system())
            .add_system(update_position.system())
            .add_system(update_state.system())
            .add_system(move_ghosts.system());
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
        .with(Target::new())
        .with(Idle)
        .with(Spawned);
}

fn move_ghosts(time: Res<Time>, board: Res<Board>, mut query: Query<With<Ghost, (&Movement, &mut Target, &mut Transform)>>) {
    for (movement, mut target, mut transform) in query.iter_mut() {
        let mut mover = Mover::new(&board, time.delta_seconds, movement, &mut target, &mut transform.translation);
        mover.move_ghost()
    }
}

fn update_position(board: Res<Board>, mut query: Query<With<Ghost, (&mut Position, &Transform)>>) {
    for (mut position, transform) in query.iter_mut() {
        *position = board.position_of_coordinates(&transform.translation);
    }
}

fn update_state(board: Res<Board>, mut query: Query<With<Ghost, (&Position, &mut State)>>) {
    for (position, mut state) in query.iter_mut() {
        if *state == Spawned && *board.type_of_position(position) == GhostWall {
            *state = Scatter
        }
    }
}

/// Set the ghosts target if he does not have one.
fn set_target(board: Res<Board>, mut query: Query<(&Ghost, &Position, &mut Target, &mut Movement, &State)>) {
    for (ghost, position, mut target, mut movement, state) in query.iter_mut() {
        if target.is_set() {
            continue;
        }

        let mut target_setter = TargetSetter::new(&board, &mut target, &mut movement, &position);
        match state {
            Spawned => target_setter.set_spawn_target(),
            Scatter => target_setter.set_scatter_target(ghost),
        }
    }
}