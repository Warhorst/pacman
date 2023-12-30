use ActiveCounter::*;
use std::collections::HashMap;
use crate::prelude::*;

pub(crate) struct Counter {
    active_counter: ActiveCounter,
    per_ghost_counter: PerGhostCounter,
    global_counter: Option<GlobalCounter>
}

impl Counter {
    pub fn new(level: &Level) -> Self {
        Counter {
            active_counter: PerGhost,
            per_ghost_counter: PerGhostCounter::new_for_level(level),
            global_counter: None
        }
    }

    pub fn increment(&mut self, current_ghost: &Ghost) {
        match self.active_counter {
            PerGhost => self.per_ghost_counter.increment(current_ghost),
            Global => self.global_counter.as_mut().unwrap().increment()
        }
    }

    pub fn switch_to_global(&mut self) {
        self.active_counter = Global;
        self.global_counter = Some(GlobalCounter::new())
    }

    /// Check if the limit for the current ghost is reached.
    ///
    /// Also switches from the global counter to the per ghost counter if the global counter is finished.
    pub fn limit_reached(&mut self, current_ghost: &Ghost) -> bool {
        match self.active_counter {
            PerGhost => self.per_ghost_counter.limit_reached_for_ghost(current_ghost),
            Global => {
                let global_counter = self.global_counter.as_ref().unwrap();
                let result = global_counter.limit_reached_for_ghost(current_ghost);
                if global_counter.is_finished() {
                    self.active_counter = PerGhost;
                }
                result
            }
        }
    }
}

enum ActiveCounter {
    PerGhost,
    Global,
}

struct PerGhostCounter {
    ghost_counter_map: HashMap<Ghost, usize>,
    ghost_limit_map: HashMap<Ghost, usize>,
}

impl PerGhostCounter {
    fn new_for_level(level: &Level) -> Self {
        let ghost_counter_map = create_ghost_value_map(0, 0, 0, 0);

        match **level {
            1 => PerGhostCounter {
                ghost_counter_map,
                ghost_limit_map: create_ghost_value_map(0, 0, 30, 60),
            },
            2 => PerGhostCounter {
                ghost_counter_map,
                ghost_limit_map: create_ghost_value_map(0, 0, 0, 50),
            },
            _ => PerGhostCounter {
                ghost_counter_map,
                ghost_limit_map: create_ghost_value_map(0, 0, 0, 0),
            }
        }
    }

    fn increment(&mut self, current_ghost: &Ghost) {
        *self.ghost_counter_map.get_mut(current_ghost).unwrap() += 1
    }

    fn limit_reached_for_ghost(&self, current_ghost: &Ghost) -> bool {
        self.ghost_counter_map.get(current_ghost).unwrap() == self.ghost_limit_map.get(current_ghost).unwrap()
    }
}

struct GlobalCounter {
    value: usize,
    ghost_limit_map: HashMap<Ghost, usize>,
}

impl GlobalCounter {
    fn new() -> Self {
        GlobalCounter {
            value: 0,
            ghost_limit_map: create_ghost_value_map(0, 7, 17, 32)
        }
    }

    fn increment(&mut self) {
        self.value += 1
    }

    fn limit_reached_for_ghost(&self, current_ghost: &Ghost) -> bool {
        *self.ghost_limit_map.get(current_ghost).unwrap() == self.value
    }

    fn is_finished(&self) -> bool {
        *self.ghost_limit_map.get(&Clyde).unwrap() == self.value
    }
}

fn create_ghost_value_map(blinky_val: usize, pinky_val: usize, inky_val: usize, clyde_val: usize) -> HashMap<Ghost, usize> {
    let mut map = HashMap::with_capacity(4);
    map.insert(Blinky, blinky_val);
    map.insert(Pinky, pinky_val);
    map.insert(Inky, inky_val);
    map.insert(Clyde, clyde_val);
    map
}