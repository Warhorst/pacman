use crate::common::Movement;
use crate::common::Movement::*;
use crate::ghosts::Target;
use crate::ghosts::target_set_strategy::TargetSetStrategy;

/// Sets the next target for a ghost.
pub(in crate::ghosts) struct TargetSetter<'a> {
    target: &'a mut Target,
    movement: &'a mut Movement
}

impl<'a> TargetSetter<'a> {
    pub fn new(target: &'a mut Target, movement: &'a mut Movement) -> Self {
        TargetSetter { target, movement }
    }

    pub fn set_target(&mut self, strategy: impl TargetSetStrategy) {
        if self.target.is_set() {
            return;
        }

        match strategy.get_next_target_neighbour() {
            Some(neighbour) => {
                self.target.set_to(neighbour.position);
                *self.movement = Moving(neighbour.direction)
            }
            None => panic!("A ghost has no new target to move to")
        }
    }
}