use crate::ghosts::target::TargetSetter;
use crate::common::Direction::*;
use crate::common::XYEqual;

impl<'a, 'b, 'c> TargetSetter<'a, 'b, 'c> {
    /// Determine the next target coordinates for a ghost when in "Eaten" state.
    ///
    /// When eaten, a ghost walks to the ghost house and enters it. When at the ghost house, he aligns perfectly
    /// before the entrance, moves than to the house center and finally to his spawn coordinates, which depend on the ghost type.
    pub fn set_eaten_target(&mut self) {
        if self.is_directly_before_entrance() {
            self.move_in_house_center()
        } else if self.is_before_entrance() {
            self.move_directly_before_entrance()
        } else if self.is_in_center() {
            self.move_to_respawn()
        } else {
            // TODO: Maybe only take this branch when not already in the ghost house, just to avoid bugs
            self.move_to_nearest_position_before_entrance()
        }
    }

    /// Return if the ghost is perfectly centered in front of the ghost house entrance.
    fn is_directly_before_entrance(&self) -> bool {
        self.components.transform.translation.xy_equal_to(&self.ghost_house.coordinates_in_front_of_entrance())
    }

    fn move_in_house_center(&mut self) {
        *self.components.direction = self.ghost_house.entrance_direction.opposite();
        self.components.target.set(self.ghost_house.center_coordinates());
    }

    /// Return if the ghost is just on a position in front of the house.
    fn is_before_entrance(&self) -> bool {
        self.ghost_house.positions_in_front_of_entrance().into_iter().any(|pos| pos == &self.dimensions.trans_to_pos(&self.components.transform))
    }

    fn move_directly_before_entrance(&mut self) {
        let in_front_of_house = self.ghost_house.coordinates_in_front_of_entrance();
        let position_coordinates = self.dimensions.pos_center(&self.components.transform.translation);

        *self.components.direction = match self.ghost_house.entrance_direction {
            Up | Down => match in_front_of_house.x < position_coordinates.x {
                true => Left,
                false => Right
            },
            Left | Right => match in_front_of_house.y < position_coordinates.y {
                true => Down,
                false => Up
            }
        };
        self.components.target.set(in_front_of_house);
    }

    fn is_in_center(&self) -> bool {
        self.components.transform.translation.xy_equal_to(&self.ghost_house.center_coordinates())
    }

    fn move_to_respawn(&mut self) {
        let center = self.ghost_house.center_coordinates();
        let respawn = self.ghost_house.respawn_coordinates_of(self.components.ghost);

        *self.components.direction = match self.ghost_house.entrance_direction {
            Up | Down => match respawn.x < center.x {
                true => Left,
                false => Right
            },
            Left | Right => match respawn.y < center.y {
                true => Down,
                false => Up
            }
        };
        self.components.target.set(respawn);
    }

    fn move_to_nearest_position_before_entrance(&mut self) {
        let nearest_spawn_position = self.dimensions.trans_to_pos(self.components.transform).get_nearest_position_from(self.ghost_house.positions_in_front_of_entrance());
        let next_target_neighbour = self.get_nearest_neighbour_to(nearest_spawn_position);
        self.set_target_to_neighbour(next_target_neighbour)
    }
}