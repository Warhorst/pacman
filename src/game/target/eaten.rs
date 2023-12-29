use crate::game::target::TargetSetter;
use crate::prelude::*;

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
        self.components.transform.translation.xy_equal(&self.get_spawn(Blinky).coordinates)
    }

    fn move_in_house_center(&mut self) {
        let pinky_spawn = *self.get_spawn(Pinky);
        *self.components.direction = pinky_spawn.spawn_direction.opposite();
        self.components.target.set(pinky_spawn.coordinates);
    }

    /// Return if the ghost is just on a position in front of the house.
    fn is_before_entrance(&self) -> bool {
        self.get_spawn(Blinky)
            .positions
            .into_iter()
            .any(|pos| pos == Pos::from_vec3(self.components.transform.translation))
    }

    fn move_directly_before_entrance(&mut self) {
        let in_front_of_house = self.get_spawn(Blinky).coordinates;
        let position_coordinates = Pos::from_vec3(self.components.transform.translation).to_vec3(0.0);

        *self.components.direction = match self.get_spawn(Pinky).spawn_direction {
            Up | Down => match in_front_of_house.x < position_coordinates.x {
                true => Left,
                false => Right
            },
            Left | Right => match in_front_of_house.y < position_coordinates.y {
                true => Down,
                false => Up
            },
        };
        self.components.target.set(in_front_of_house);
    }

    fn is_in_center(&self) -> bool {
        self.components.transform.translation.xy_equal(&self.get_spawn(Pinky).coordinates)
    }

    fn move_to_respawn(&mut self) {
        let center = self.get_spawn(Pinky).coordinates;
        let respawn = match *self.components.ghost {
            Blinky => self.get_spawn(Pinky).coordinates,
            _ => self.get_spawn(*self.components.ghost).coordinates
        };

        *self.components.direction = match self.get_spawn(Pinky).spawn_direction {
            Up | Down => match respawn.x < center.x {
                true => Left,
                false => Right
            },
            Left | Right => match respawn.y < center.y {
                true => Down,
                false => Up
            },
        };
        self.components.target.set(respawn);
    }

    fn move_to_nearest_position_before_entrance(&mut self) {
        let position = Pos::from_vec3(self.components.transform.translation);
        let nearest_spawn_position = self.get_spawn(Blinky)
            .positions
            .into_iter()
            .map(|pos| (pos, pos.distance(&position)))
            .min_by(|(_, dis_a), (_, dis_b)| dis_a.partial_cmp(&dis_b).unwrap())
            .map(|(pos, _)| pos)
            .unwrap();

        let next_target_neighbour = self.get_nearest_neighbour_to(nearest_spawn_position);
        self.set_target_to_neighbour(next_target_neighbour)
    }
}