use bevy::prelude::*;
use crate::common::position::Position;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::map::board::Board;
use crate::common::Direction::*;

pub struct BoardDimensions {
    field_dimension: f32,
    board_origin: Vec2,
    board_width: f32
}

impl BoardDimensions {
    pub fn new(board: &Board) -> Self {
        let board_columns = board.width as f32;
        let board_rows = board.height as f32;
        let field_dimension = (WINDOW_HEIGHT * 0.8) / board_rows;

        BoardDimensions {
            field_dimension,
            board_origin: Vec2::new(
                WINDOW_WIDTH / 2.0 - (board_columns * field_dimension) / 2.0,
                WINDOW_HEIGHT * 0.1
            ),
            board_width: board_columns * field_dimension
        }
    }

    pub fn field(&self) -> f32 {
        self.field_dimension
    }

    pub fn wall(&self) -> f32 {
        self.field_dimension
    }

    pub fn pacman(&self) -> f32 {
        self.field_dimension + self.field_dimension * 0.6
    }

    pub fn ghost(&self) -> f32 {
        self.pacman()
    }

    pub fn dot(&self) -> f32 {
        self.pacman()
    }

    pub fn energizer(&self) -> f32 {
        self.pacman()
    }

    pub fn fruit(&self) -> f32 {
        self.pacman()
    }

    pub fn tunnel(&self) -> f32 {
        self.pacman()
    }

    pub fn life(&self) -> f32 {
        self.pacman()
    }

    pub fn pacman_base_speed(&self) -> f32 {
        self.field_dimension * 9.0
    }

    pub fn ghost_base_speed(&self) -> f32 {
        self.pacman_base_speed()
    }

    pub fn pos_to_vec(&self, pos: &Position, z: f32) -> Vec3 {
        Vec3::new(
            self.board_origin.x + self.field_dimension * pos.x as f32,
            self.board_origin.y + self.field_dimension * pos.y as f32,
            z
        )
    }

    pub fn pos_to_trans(&self, pos: &Position, z: f32) -> Transform {
        Transform::from_translation(self.pos_to_vec(pos, z))
    }

    pub fn positions_to_vec<'a>(&'a self, positions: impl IntoIterator<Item=&'a Position>, z: f32) -> Vec3 {
        let positions = positions.into_iter().collect::<Vec<_>>();
        assert_eq!(positions.len(), 2);

        let (pos0, pos1) = (positions[0], positions[1]);
        let neighbour_direction = pos0.get_neighbour_direction(&pos1).expect("the two positions must be neighbored");
        let (vec0, vec1) = (self.pos_to_vec(pos0, 0.0), self.pos_to_vec(pos1, 0.0));

        match neighbour_direction {
            Up | Down => {
                let x = vec0.x;
                let y = (vec0.y + vec1.y) / 2.0;
                Vec3::new(x, y, 0.0)
            }
            Left | Right => {
                let x = (vec0.x + vec1.x) / 2.0;
                let y = vec0.y;
                Vec3::new(x, y, z)
            }
        }
    }

    pub fn positions_to_trans<'a>(&'a self, positions: impl IntoIterator<Item=&'a Position>, z: f32) -> Transform {
        Transform::from_translation(self.positions_to_vec(positions, z))
    }

    pub fn vec_to_pos(&self, vec: &Vec3) -> Position {
        let x = (vec.x - self.board_origin.x + self.field_dimension / 2.0) / self.field_dimension;
        let y = (vec.y - self.board_origin.y + self.field_dimension / 2.0) / self.field_dimension;

        Position::new(
            x as isize,
            y as isize
        )
    }

    pub fn trans_to_pos(&self, transform: &Transform) -> Position {
        self.vec_to_pos(&transform.translation)
    }

    pub fn pos_center(&self, vec: &Vec3) -> Vec3 {
        self.pos_to_vec(&self.vec_to_pos(vec), vec.z)
    }

    pub fn origin(&self) -> Vec2 {
        self.board_origin
    }

    pub fn board_width(&self) -> f32 {
        self.board_width
    }

    pub fn board_height(&self) -> f32 {
        WINDOW_HEIGHT * 0.8
    }
}