use std::any::TypeId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};

pub type MovementTextures = HashMap<Direction, Handle<Image>>;

/// Resource that holds all handles to ghost textures.
pub struct GhostTextures {
    pub normal_ghost_textures: HashMap<TypeId, MovementTextures>,
}

impl GhostTextures {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut normal_ghost_textures = HashMap::with_capacity(4);
        normal_ghost_textures.insert(TypeId::of::<Blinky>(), Self::load_movement_textures_for("blinky", asset_server));
        normal_ghost_textures.insert(TypeId::of::<Pinky>(), Self::load_movement_textures_for("pinky", asset_server));
        normal_ghost_textures.insert(TypeId::of::<Inky>(), Self::load_movement_textures_for("inky", asset_server));
        normal_ghost_textures.insert(TypeId::of::<Clyde>(), Self::load_movement_textures_for("clyde", asset_server));

        GhostTextures { normal_ghost_textures }
    }

    fn load_movement_textures_for(ghost: &str, asset_server: &AssetServer) -> MovementTextures {
        let mut textures = HashMap::with_capacity(4);
        textures.insert(Up, asset_server.load(&format!("textures/ghost/{ghost}_up.png")));
        textures.insert(Down, asset_server.load(&format!("textures/ghost/{ghost}_down.png")));
        textures.insert(Left, asset_server.load(&format!("textures/ghost/{ghost}_left.png")));
        textures.insert(Right, asset_server.load(&format!("textures/ghost/{ghost}_right.png")));
        textures
    }

    // TODO is cloning the right way?
    pub fn get_normal_texture_for<G: 'static + GhostType>(&self, direction: &Direction) -> Handle<Image> {
        self.normal_ghost_textures
            .get(&TypeId::of::<G>())
            .expect("Ghost should have textures")
            .get(direction)
            .expect("texture for direction should be present")
            .clone()
    }
}