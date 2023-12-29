use std::collections::HashMap;
use bevy::prelude::*;
use crate::prelude::*;

pub (in crate::game) struct SpecsPerLevelPlugin;

impl Plugin for SpecsPerLevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(create_specs_per_level())
        ;
    }
}

fn create_specs_per_level() -> SpecsPerLevel {
    SpecsPerLevel::from_levels_and_specs(
        [
            (1, Spec {
                fruit_to_spawn: Cherry,
                pacman_normal_speed_modifier: 0.8,
                pacman_frightened_speed_modifier: 0.9,
                ghost_normal_speed_modifier: 0.75,
                ghost_tunnel_speed_modifier: 0.4,
                ghost_frightened_speed_modifier: 0.5,
                elroy_1_dots_left: 20,
                elroy_1_speed_modifier: 0.8,
                elroy_2_dots_left: 10,
                elroy_2_speed_modifier: 0.85,
                frightened_time: 6.0,
            }),
            (2, Spec {
                fruit_to_spawn: Strawberry,
                pacman_normal_speed_modifier: 0.9,
                pacman_frightened_speed_modifier: 0.95,
                ghost_normal_speed_modifier: 0.85,
                ghost_tunnel_speed_modifier: 0.45,
                ghost_frightened_speed_modifier: 0.55,
                elroy_1_dots_left: 30,
                elroy_1_speed_modifier: 0.9,
                elroy_2_dots_left: 15,
                elroy_2_speed_modifier: 0.95,
                frightened_time: 5.0,
            }),
            (3, Spec {
                fruit_to_spawn: Peach,
                pacman_normal_speed_modifier: 0.9,
                pacman_frightened_speed_modifier: 0.95,
                ghost_normal_speed_modifier: 0.85,
                ghost_tunnel_speed_modifier: 0.45,
                ghost_frightened_speed_modifier: 0.55,
                elroy_1_dots_left: 40,
                elroy_1_speed_modifier: 0.9,
                elroy_2_dots_left: 20,
                elroy_2_speed_modifier: 0.95,
                frightened_time: 4.0,
            }),
            (4, Spec {
                fruit_to_spawn: Peach,
                pacman_normal_speed_modifier: 0.9,
                ghost_normal_speed_modifier: 0.85,
                ghost_tunnel_speed_modifier: 0.45,
                elroy_1_dots_left: 40,
                elroy_1_speed_modifier: 0.9,
                elroy_2_dots_left: 20,
                elroy_2_speed_modifier: 0.95,
                pacman_frightened_speed_modifier: 0.95,
                ghost_frightened_speed_modifier: 0.55,
                frightened_time: 3.0,
            }),
            (5, Spec {
                fruit_to_spawn: Apple,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 40,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 20,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 2.0,
            }),
            (6, Spec {
                fruit_to_spawn: Apple,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 50,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 25,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 5.0,
            }),
            (7, Spec {
                fruit_to_spawn: Grapes,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 50,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 25,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 2.0,
            }),
            (8, Spec {
                fruit_to_spawn: Grapes,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 50,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 25,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 2.0,
            }),
            (9, Spec {
                fruit_to_spawn: Galaxian,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 60,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 30,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (10, Spec {
                fruit_to_spawn: Galaxian,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 60,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 30,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 5.0,
            }),
            (11, Spec {
                fruit_to_spawn: Bell,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 60,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 30,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 2.0,
            }),
            (12, Spec {
                fruit_to_spawn: Bell,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 80,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 40,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (13, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 80,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 40,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (14, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 80,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 40,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 3.0,
            }),
            (15, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 100,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 50,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (16, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 100,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 50,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (17, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 100,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 50,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.95,
                frightened_time: 0.0,
            }),
            (18, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 100,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 50,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.6,
                frightened_time: 1.0,
            }),
            (19, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 120,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 60,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.95,
                frightened_time: 0.0,
            }),
            (20, Spec {
                fruit_to_spawn: Key,
                pacman_normal_speed_modifier: 1.0,
                ghost_normal_speed_modifier: 0.95,
                ghost_tunnel_speed_modifier: 0.5,
                elroy_1_dots_left: 120,
                elroy_1_speed_modifier: 1.0,
                elroy_2_dots_left: 60,
                elroy_2_speed_modifier: 1.05,
                pacman_frightened_speed_modifier: 1.0,
                ghost_frightened_speed_modifier: 0.95,
                frightened_time: 0.0,
            })
        ],
        Spec {
            fruit_to_spawn: Key,
            pacman_normal_speed_modifier: 0.9,
            pacman_frightened_speed_modifier: 0.9,
            ghost_normal_speed_modifier: 0.95,
            ghost_tunnel_speed_modifier: 0.5,
            ghost_frightened_speed_modifier: 0.95,
            elroy_1_dots_left: 120,
            elroy_1_speed_modifier: 1.0,
            elroy_2_dots_left: 60,
            elroy_2_speed_modifier: 1.05,
            frightened_time: 0.0,
        }
    )
}

#[derive(Resource)]
pub struct SpecsPerLevel {
    level_to_spec: HashMap<Level, Spec>,
    default: Spec
}

impl SpecsPerLevel {
    fn from_levels_and_specs(levels_and_specs: impl IntoIterator<Item=(usize, Spec)>, default: Spec) -> Self {
        SpecsPerLevel {
            level_to_spec: levels_and_specs
                .into_iter()
                .map(|(l, s)| (Level(l), s))
                .collect(),
            default
        }
    }

    pub fn get_for(&self, level: &Level) -> &Spec {
        self.level_to_spec.get(level).unwrap_or(&self.default)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Spec {
    pub fruit_to_spawn: Fruit,
    pub pacman_normal_speed_modifier: f32,
    pub pacman_frightened_speed_modifier: f32,
    pub ghost_normal_speed_modifier: f32,
    pub ghost_tunnel_speed_modifier: f32,
    pub ghost_frightened_speed_modifier: f32,
    pub elroy_1_dots_left: usize,
    pub elroy_1_speed_modifier: f32,
    pub elroy_2_dots_left: usize,
    pub elroy_2_speed_modifier: f32,
    pub frightened_time: f32,
}

#[cfg(test)]
mod tests {
    use crate::game::edibles::fruit::Fruit;
    use crate::game::level::Level;
    use crate::game::specs_per_level::{Spec, SpecsPerLevel};

    #[test]
    fn it_can_be_created_from_an_iterator_of_usize_spec_tuples() {
        let specs_per_level = SpecsPerLevel::from_levels_and_specs(
            [
                (1, Spec {
                    fruit_to_spawn: Fruit::Cherry,
                    pacman_normal_speed_modifier: 1.0,
                    pacman_frightened_speed_modifier: 1.0,
                    ghost_normal_speed_modifier: 1.0,
                    ghost_tunnel_speed_modifier: 1.0,
                    ghost_frightened_speed_modifier: 1.0,
                    elroy_1_dots_left: 1,
                    elroy_1_speed_modifier: 1.0,
                    elroy_2_dots_left: 1,
                    elroy_2_speed_modifier: 1.0,
                    frightened_time: 1.0,
                }),
                (2, Spec {
                    fruit_to_spawn: Fruit::Apple,
                    pacman_normal_speed_modifier: 2.0,
                    pacman_frightened_speed_modifier: 2.0,
                    ghost_normal_speed_modifier: 2.0,
                    ghost_tunnel_speed_modifier: 2.0,
                    ghost_frightened_speed_modifier: 2.0,
                    elroy_1_dots_left: 2,
                    elroy_1_speed_modifier: 2.0,
                    elroy_2_dots_left: 2,
                    elroy_2_speed_modifier: 2.0,
                    frightened_time: 2.0,
                })
            ],
            Spec::default()
        );

        assert_eq!(specs_per_level.level_to_spec.len(), 2);
    }

    #[test]
    fn it_returns_the_right_spec_for_a_given_level() {
        let level = Level(1);
        let spec = Spec::default();

        let specs_per_level = SpecsPerLevel::from_levels_and_specs(
            [(1, spec.clone())],
            Spec::default()
        );

        let retrieved_spec = specs_per_level.get_for(&level);

        assert_eq!(retrieved_spec, &spec)
    }

    #[test]
    fn it_returns_a_default_spec_when_none_is_registered_for_the_given_level() {
        let spec = Spec {
            fruit_to_spawn: Fruit::Cherry,
            pacman_normal_speed_modifier: 1.0,
            pacman_frightened_speed_modifier: 1.0,
            ghost_normal_speed_modifier: 1.0,
            ghost_tunnel_speed_modifier: 1.0,
            ghost_frightened_speed_modifier: 1.0,
            elroy_1_dots_left: 1,
            elroy_1_speed_modifier: 1.0,
            elroy_2_dots_left: 1,
            elroy_2_speed_modifier: 1.0,
            frightened_time: 1.0,
        };

        let specs_per_level = SpecsPerLevel::from_levels_and_specs(
            [(1, spec.clone())],
            Spec::default()
        );

        let retrieved_spec = specs_per_level.get_for(&Level(42));

        assert_eq!(retrieved_spec, &specs_per_level.default)
    }
}