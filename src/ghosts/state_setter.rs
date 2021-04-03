use crate::common::Position;
use crate::ghosts::components::{Schedule, State};
use crate::map::board::Board;
use crate::map::FieldType::GhostWall;

use super::components::State::*;

pub struct StateSetter<'a> {
    state: &'a mut State,
    position: &'a Position,
    schedule: &'a mut Schedule,
    board: &'a Board,
    delta_time: f32,
}

impl<'a> StateSetter<'a> {
    pub fn new(state: &'a mut State, position: &'a Position, schedule: &'a mut Schedule, board: &'a Board, delta_time: f32) -> Self {
        StateSetter { state, position, schedule, board, delta_time }
    }

    pub fn set_next_state(&mut self) {
        match self.state {
            Frightened | Eaten => return,
            Spawned => if self.board.type_of_position(self.position) == &GhostWall {
                self.update_and_set_state()
            }
            _ => self.update_and_set_state()
        }
    }

    fn update_and_set_state(&mut self) {
        *self.state = self.schedule.state_after_tick(self.delta_time)
    }
}