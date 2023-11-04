use std::fs::File;
use std::io::Write;
use bevy::prelude::*;
use crate::game::pacman::Pacman;

pub struct MapCreator {
    world: World
}

impl MapCreator {
    pub fn new(app: App) -> Self {
        let mut world = World::new();
        let type_registry = app.world.resource::<AppTypeRegistry>().clone();
        world.insert_resource(type_registry);

        MapCreator {
            world
        }
    }

    pub fn create_map(&mut self) {
        let mut app = App::new();
        app.register_type::<Pacman>();
        let type_registry = app.world.resource::<AppTypeRegistry>().clone();
        self.world.insert_resource(type_registry);

        self.world.spawn(Pacman);
    }

    pub fn store_as_scene(&mut self) {
        let scene = DynamicScene::from_world(&self.world);
        let type_registry = self.world.resource::<AppTypeRegistry>();
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();

        File::create("./map/default_map.scn.ron")
            .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            .expect("Error while writing scene to file");
    }
}