use bevy::prelude::*;

use std::any::TypeId;
use std::collections::HashMap;
use crate::level::Level;

/// Resource that tells if ghost can leave the ghost house
pub struct GhostHouseGate {
    counter_per_ghost: HashMap<TypeId, usize>,
    global_counter: Option<usize>,
    release_timer: Timer
}

impl GhostHouseGate {

}