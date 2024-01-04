use std::fs::File;
use std::io::Write;
use bevy::prelude::*;
use crate::core::prelude::*;

/// Create the default pacman map as a bevy scene.
/// TODO ghost corners
pub fn create_map(commands: &mut App) {
    let mut creator = MapCreator::new(28, 31, commands);
    creator.create();
    creator.save()
}

macro_rules! corner {
    ($creator:expr, $rot:expr, O) => {
        $creator.spawn($creator.maze, (
            Wall,
            WallStyle {
                wall_type: WallType::Outer,
                rotation: $rot,
                is_corner: true,
            }
        ))
    };

    ($creator:expr, $rot:expr, O, $ghost:expr) => {
        $creator.spawn($creator.maze, (
            Wall,
            WallStyle {
                wall_type: WallType::Outer,
                rotation: $rot,
                is_corner: true,
            },
            GhostCorner($ghost)
        ))
    };

    ($creator:expr, $rot:expr, I) => {
        $creator.spawn($creator.maze, (
            Wall,
            WallStyle {
                wall_type: WallType::Inner,
                rotation: $rot,
                is_corner: true,
            }
        ))
    };
}

macro_rules! wall {
    ($creator:expr, $amount:expr, $rot:expr, O) => {
        for _ in 0..$amount {
            $creator.spawn($creator.maze, (
                Wall,
                WallStyle {
                    wall_type: WallType::Outer,
                    rotation: $rot,
                    is_corner: false,
                }
            ))
        }
    };

    ($creator:expr, $amount:expr, $rot:expr, I) => {
        for _ in 0..$amount {
            $creator.spawn($creator.maze, (
                Wall,
                WallStyle {
                    wall_type: WallType::Inner,
                    rotation: $rot,
                    is_corner: false,
                }
            ))
        }
    };
}

macro_rules! dot {
    ($creator:expr, $amount:expr) => {
        for _ in 0..$amount {
            $creator.spawn($creator.dot_spawns, (
                DotSpawn,
            ))
        }
    };
}

macro_rules! energizer {
    ($creator:expr) => {
        $creator.spawn(
            $creator.energizer_spawns,
            EnergizerSpawn
        );
    };
}

macro_rules! empty {
    ($creator:expr, $amount:expr) => {
        for _ in 0..$amount {
            $creator.cont()
        }
    };
}

macro_rules! ghost_house {
    ($creator:expr, $amount:expr) => {
        for _ in 0..$amount {
            $creator.spawn(
                $creator.maze,
                GhostHouseArea {rotation: D0}
            );
        }
    };
}

macro_rules! tunnel_left {
    ($creator:expr) => {
        $creator.spawn(
            $creator.maze,
            Tunnel {direction: Left, index: 0}
        );

        for _ in 0..5 {
            $creator.spawn(
                $creator.maze,
                TunnelHallway
            );
        }
    };
}

macro_rules! tunnel_right {
    ($creator:expr) => {
        for _ in 0..5 {
            $creator.spawn(
                $creator.maze,
                TunnelHallway
            );
        }

        $creator.spawn(
            $creator.maze,
            Tunnel {direction: Right, index: 0}
        );
    };
}

macro_rules! fruit {
    ($creator:expr) => {
        $creator.spawn_double(
            $creator.map,
            FruitSpawn
        );
    };
}

macro_rules! pacman {
    ($creator:expr) => {
        $creator.spawn_double(
            $creator.map,
            PacmanSpawn
        );
    };
}

struct MapCreator<'a> {
    /// Max width (or column) of the map
    width: usize,
    /// Current column in the spawn process
    current_column: usize,
    /// Current row in the spawn process
    current_row: usize,
    /// The app which has all types registered for reflection
    app: &'a mut App,
    /// The world which forms the map scene. Used for spawning the entities and storing the scene
    map_world: World,
    /// Parent entity of the entire map
    map: Entity,
    /// Parent entity of all walls
    maze: Entity,
    /// Parent entity of all dot spawns
    dot_spawns: Entity,
    /// Parent entity of all energizer spawns
    energizer_spawns: Entity
}

impl<'a> MapCreator<'a> {
    fn new(width: usize, height: usize, app: &'a mut App) -> Self {
        let mut map_world = World::new();
        let map = map_world.spawn(Map { width, height }).id();
        let maze = map_world.spawn(Maze).id();
        let dot_spawns = map_world.spawn(DotSpawns).id();
        let energizer_spawns = map_world.spawn(EnergizerSpawns).id();

        map_world.entity_mut(map).push_children(&[maze, dot_spawns, energizer_spawns]);

        MapCreator {
            width,
            current_column: 0,
            current_row: height,
            app,
            map_world,
            map,
            maze,
            dot_spawns,
            energizer_spawns
        }
    }

    fn create(&mut self) {
        // 0
        corner!(self, D0, O, Pinky);
        wall!(self, 12, D0, O);
        corner!(self, D90, O);
        corner!(self, D0, O);
        wall!(self, 12, D0, O);
        corner!(self, D90, O, Blinky);

        // 1
        wall!(self, 1, D270, O);
        dot!(self, 12);
        wall!(self, 1, D90, O);
        wall!(self, 1, D270, O);
        dot!(self, 12);
        wall!(self, 1, D90, O);

        // 2
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 3, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 3, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);

        // 3
        wall!(self, 1, D270, O);
        energizer!(self);
        wall!(self, 1, D270, I);
        empty!(self, 2);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        empty!(self, 3);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D90, I);
        empty!(self, 3);
        wall!(self, 1, D270, I);
        dot!(self, 1);
        wall!(self, 1, D90, I);
        empty!(self, 2);
        wall!(self, 1, D270, I);
        energizer!(self);
        wall!(self, 1, D90, O);
        
        // 4
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 3, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, O);
        corner!(self, D180, O);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 3, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 5
        wall!(self, 1, D270, O);
        dot!(self, 26);
        wall!(self, 1, D90, O);
        
        // 6
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 6, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);

        // 7
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D90, I);
        corner!(self, D0, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 8
        wall!(self, 1, D270, O);
        dot!(self, 6);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 4);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 4);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 6);
        wall!(self, 1, D90, O);
        
        // 9
        corner!(self, D270, O);
        wall!(self, 4, D180, O);
        corner!(self, D90, O);
        dot!(self, 1);
        wall!(self, 1, D90, I);
        corner!(self, D270, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        empty!(self, 1);
        wall!(self, 2, D90, I);
        empty!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D180, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D0, O);
        wall!(self, 4, D180, O);
        corner!(self, D180, O);
        
        // 10
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D90, I);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D180, I);
        empty!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        empty!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);
        
        // 11
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        empty!(self, 10);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);

        // 12
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        empty!(self, 1);
        ghost_house!(self, 8);
        empty!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);
        
        // 13
        wall!(self, 5, D180, O);
        corner!(self, D180, O);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        empty!(self, 1);
        ghost_house!(self, 8);
        empty!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, O);
        wall!(self, 5, D180, O);
        
        // 14
        tunnel_left!(self);
        empty!(self, 4);
        ghost_house!(self, 8);
        empty!(self, 4);
        tunnel_right!(self);
        
        // 15
        wall!(self, 5, D0, O);
        corner!(self, D90, O);
        dot!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        empty!(self, 1);
        ghost_house!(self, 8);
        empty!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, O);
        wall!(self, 5, D0, O);
        
        // 16
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        empty!(self, 1);
        ghost_house!(self, 8);
        empty!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);
        
        // 17
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        empty!(self, 4);
        fruit!(self);
        empty!(self, 4);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);
        
        // 18
        empty!(self, 5);
        wall!(self, 1, D270, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        empty!(self, 1);
        corner!(self, D0, I);
        wall!(self, 6, D0, I);
        corner!(self, D90, I);
        empty!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        empty!(self, 5);
        
        // 19
        corner!(self, D0, O);
        wall!(self, 4, D180, O);
        corner!(self, D180, O);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        empty!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D90, I);
        corner!(self, D0, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        empty!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, O);
        wall!(self, 4, D180, O);
        corner!(self, D90, O);
        
        // 20
        wall!(self, 1, D270, O);
        dot!(self, 12);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 12);
        wall!(self, 1, D90, O);
        
        // 21
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 3, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 3, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 22
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 1, D0, I);
        corner!(self, D90, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 3, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 3, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D90, I);
        corner!(self, D0, I);
        wall!(self, 1, D0, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 23
        wall!(self, 1, D270, O);
        energizer!(self);
        dot!(self, 2);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 7);
        pacman!(self);
        dot!(self, 7);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 2);
        energizer!(self);
        wall!(self, 1, D90, O);
        
        // 24
        corner!(self, D270, O);
        wall!(self, 1, D0, O);
        corner!(self, D90, O);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 6, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D0, O);
        wall!(self, 1, D0, O);
        corner!(self, D180, O);
        
        // 25
        corner!(self, D0, O);
        wall!(self, 1, D180, O);
        corner!(self, D180, O);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 2, D180, I);
        corner!(self, D90, I);
        corner!(self, D0, I);
        wall!(self, 2, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, O);
        wall!(self, 1, D180, O);
        corner!(self, D90, O);
        
        // 26
        wall!(self, 1, D270, O);
        dot!(self, 6);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 4);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 4);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 6);
        wall!(self, 1, D90, O);
        
        // 27
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 4, D0, I);
        corner!(self, D180, I);
        corner!(self, D270, I);
        wall!(self, 2, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D270, I);
        wall!(self, 1, D90, I);
        dot!(self, 1);
        corner!(self, D0, I);
        wall!(self, 2, D0, I);
        corner!(self, D180, I);
        corner!(self, D270, I);
        wall!(self, 4, D0, I);
        corner!(self, D90, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 28
        wall!(self, 1, D270, O);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 8, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        corner!(self, D180, I);
        dot!(self, 1);
        corner!(self, D270, I);
        wall!(self, 8, D180, I);
        corner!(self, D180, I);
        dot!(self, 1);
        wall!(self, 1, D90, O);
        
        // 29
        wall!(self, 1, D270, O);
        dot!(self, 26);
        wall!(self, 1, D90, O);

        // 30
        corner!(self, D270, O, Clyde);
        wall!(self, 26, D180, O);
        corner!(self, D180, O, Inky);
    }

    fn spawn(&mut self, parent: Entity, bundle: impl Bundle) {
        let e = self.map_world.spawn((
            bundle,
            Tiles::Single { pos: Pos::new(self.current_column as isize, self.current_row as isize) }
        )).id();

        self.map_world.entity_mut(parent).push_children(&[e]);

        self.cont()
    }

    fn spawn_double(&mut self, parent: Entity, bundle: impl Bundle) {
        let e = self.map_world.spawn((
            bundle,
            Tiles::Double {
                pos_a: Pos::new(self.current_column as isize, self.current_row as isize),
                pos_b: Pos::new((self.current_column + 1) as isize, self.current_row as isize),
            }
        )).id();

        self.map_world.entity_mut(parent).push_children(&[e]);

        self.cont();
        self.cont();
    }

    /// Continue to next pos
    fn cont(&mut self) {
        self.current_column += 1;

        if self.current_column == self.width {
            self.current_column = 0;
            self.current_row -= 1;
        }
    }

    fn save(&mut self) {
        let type_registry = self.app.world.resource::<AppTypeRegistry>().clone();
        self.map_world.insert_resource(type_registry);

        let scene = DynamicScene::from_world(&self.map_world);

        let type_registry = self.app.world.resource::<AppTypeRegistry>();
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();
        File::create(format!("./assets/{MAP_SCENE_PATH}"))
            .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            .expect("error while writing map to file");
    }
}