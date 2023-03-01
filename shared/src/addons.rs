use std::{collections::HashMap, path::Path};

use mlua::{Lua, Table};

use crate::{resources, direction::Direction, types::{Id, Type}};

// TODO: Addon toml settings and addon priority for feature overrides
pub struct AddonManager {
    addons: Vec<Addon>,
    data: HashMap<Id, Lua>,
}

impl Default for AddonManager {
    fn default() -> Self {
        Self {
            addons: vec![],
            data: HashMap::new(),
        }
    }
}

impl AddonManager {
    #[profiling::function]
    pub fn load(addon_path: impl AsRef<Path>) -> Self {
        let mut manager = Self::default();

        for entry in resources::read_dir(addon_path).unwrap() {
            let addon = Addon::new();
            manager.addons.push(addon);
            
            manager.load_blocks(entry.unwrap().path());
            // TODO: Make load items function
        }

        manager
    }

    #[profiling::function]
    pub fn reload(&mut self, addon_path: impl AsRef<Path>) {
        *self = Self::load(addon_path);
    }

    #[profiling::function]
    pub fn get(&self, index: &Id) -> Table {
        let lua = self.data.get(index).expect(&format!("Addon content index {:?} does not exist", index));
        lua.globals().get("Block").unwrap()
    }

    #[profiling::function]
    pub fn get_block_texture(&self, index: &Id, direction: &Direction) -> String {
        let block = self.get(index);
        let textures: Table = block.get("textures").unwrap();

        fn face(direction: String, textures: &Table, side: bool) -> String {
            if let Ok(up) = textures.get(direction) {
                return up;
            }
            if side {
                if let Ok(side) = textures.get("side") {
                    return side;
                }
            }
            textures.get("all").unwrap()
        }

        match direction {
            Direction::UP => face(direction.get_string(), &textures, false),
            Direction::DOWN => face(direction.get_string(), &textures, false),
            direction => face(direction.get_string(), &textures, true),
        }
    }

    #[profiling::function]
    fn load_blocks(&mut self, path: impl AsRef<Path>) { // TODO: More verbose errors
        for entry in resources::read_dir(path.as_ref().join("blocks")).unwrap() {
            let Ok(code) = resources::read_dir_entry_string(entry.as_ref().unwrap(), Some("lua")) else { continue };

            if entry.as_ref().unwrap().file_name() == "template.lua" {
                continue
            }

            let lua = Lua::new();
            
            // Create block variable
            let block = lua.create_table().unwrap();

            // Declare placement state enum
            let placement_state = lua.create_table().unwrap();
            placement_state.set("Random", 0).unwrap();
            placement_state.set("Facing", 1).unwrap();
            placement_state.set("FacingAway", 2).unwrap();

            // Declare pathfinding state enum
            let pathfinding_state = lua.create_table().unwrap();
            pathfinding_state.set("Solid", 0).unwrap();
            pathfinding_state.set("Dangerous", 1).unwrap();
            pathfinding_state.set("Empty", 2).unwrap();

            // Add variables to lua scripts
            lua.globals().set("Block", block).unwrap();
            lua.globals().set("PlacementState", placement_state).unwrap();
            lua.globals().set("PathfindingState", pathfinding_state).unwrap();

            // Get code and run it
            lua.load(&code).exec().unwrap();

            // Parse output
            let block: Table = lua.globals().get("Block").unwrap();
            let namespace = block.get("namespace").unwrap();
            let id = block.get("id").unwrap();
            drop(block);

            self.data.insert(Id::new(Type::Block, namespace, id), lua);
        }
    }
}

struct Addon {
}

impl Addon {
    fn new() -> Self {
        Self {
        }
    }
}