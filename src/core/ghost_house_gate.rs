use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;

use ActiveCounter::*;

use crate::prelude::*;

/// Resource that tells if ghost can leave the ghost house.
///
/// This is the most complex piece of logic in this entire game. It basically works like this:
///
/// There are two types of counters: one which counts per ghost and a global counter. At the beginning
/// of the game, the per ghost counter is set to zero for each ghost and the global counter is inactive.
/// If pacman eats a dot, the current counter gets incremented.
///
/// If the per ghost counter is active, a ghost can leave if its personal limit is reached. Only
/// the counter from the currently waiting ghost is incremented.
///
/// The order of preference for ghosts is Blinky, Pinky, Inky and Clyde. Blinky and Pinky can always
/// leave the house at the beginning of the game.
///
/// If pacman dies, the per ghost counter is switched with a newly initialized global one (while retaining
/// the per ghost one). The waiting ghost can now leave when its predefined limit is reached. When
/// Clyde left the house, the counter switches back to the per ghost one.
///
/// There is also a timer active. If the timer reaches zero, the waiting ghost can return immediately.
/// The timer gets reset when pacman eats a dot.
#[derive(Resource)]
pub struct GhostHouseGate {
    released_ghosts: HashSet<Ghost>,
    ghost_preference_iterator: GhostPreferenceIterator,
    current_waiting_ghost: Ghost,
    counter: Counter,
    release_timer: Timer,
}

impl GhostHouseGate {
    pub fn new(level: &Level) -> Self {
        let mut iterator = GhostPreferenceIterator::new();
        let current_waiting_ghost = iterator.next().unwrap();

        GhostHouseGate {
            released_ghosts: HashSet::with_capacity(NUM_GHOST_TYPES),
            ghost_preference_iterator: iterator,
            current_waiting_ghost,
            counter: Counter::new(level),
            release_timer: Self::create_release_timer_for_level(level),
        }
    }

    fn create_release_timer_for_level(level: &Level) -> Timer {
        match **level {
            l if l < 5 => Timer::from_seconds(4.0, TimerMode::Once),
            _ => Timer::from_seconds(3.0, TimerMode::Once)
        }
    }

    /// Ask the gate if the given ghost type can be released.
    pub fn ghost_can_leave_house(&self, ghost: &Ghost) -> bool {
        self.released_ghosts.contains(ghost)
    }

    /// Increment the current counter. Typically when a dot was eaten.
    /// Also resets the release timer.
    pub fn increment_counter(&mut self) {
        self.release_timer.reset();
        self.counter.increment(&self.current_waiting_ghost)
    }

    /// Switch to the global counter. Typically called when pacman died.
    pub fn switch_to_global_counter(&mut self) {
        self.counter.switch_to_global();
        self.release_timer.reset();
        self.released_ghosts.clear();
        self.ghost_preference_iterator = GhostPreferenceIterator::new();
        self.current_waiting_ghost = self.ghost_preference_iterator.next().expect("first item should exists");
    }

    /// Proceed the release timer and check if the current waiting ghost can be released.
    pub fn update(&mut self, delta: Duration) {
        if self.all_ghosts_released() { return; }

        self.release_timer.tick(delta);

        if self.release_timer.finished() {
            self.release_timer.reset();
            self.release_current_waiting_ghost();
        } else if self.counter.limit_reached(&self.current_waiting_ghost) {
            self.release_current_waiting_ghost()
        }
    }

    fn all_ghosts_released(&self) -> bool {
        self.released_ghosts.len() == NUM_GHOST_TYPES
    }

    fn release_current_waiting_ghost(&mut self) {
        self.released_ghosts.insert(self.current_waiting_ghost);

        if let Some(id) = self.ghost_preference_iterator.next() {
            self.current_waiting_ghost = id
        }
    }
}

struct GhostPreferenceIterator {
    ghost_preferences: [Ghost; NUM_GHOST_TYPES],
    current: usize,
}

impl GhostPreferenceIterator {
    fn new() -> Self {
        GhostPreferenceIterator {
            ghost_preferences: [Blinky, Pinky, Inky, Clyde],
            current: 0,
        }
    }
}

impl Iterator for GhostPreferenceIterator {
    type Item = Ghost;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            c if c >= NUM_GHOST_TYPES => None,
            ref mut c => {
                let next = self.ghost_preferences[*c];
                *c += 1;
                Some(next)
            }
        }
    }
}

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