use std::{collections::HashMap, path::Path};

use mlua::{Lua, Table};

use crate::{resources, InnerModule};

// TODO: Addon toml settings and addon priority for feature overrides

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Block(String, u32),
    Item(String, u32),
}

pub struct AddonManager {
    addons: Vec<Addon>,
    data: HashMap<Type, Lua>,
}

impl AddonManager {
    pub fn new() -> Self {
        Self {
            addons: vec![],
            data: HashMap::new(),
        }
    }

    pub fn load(&mut self, addon_path: impl AsRef<Path>) {
        for entry in resources::read_dir(addon_path).unwrap() {
            let addon = Addon::new();
            self.addons.push(addon);
            
            self.load_blocks(entry.unwrap().path());
            // TODO: Make load items function
        }
    }

    pub fn reload(&mut self, addon_path: impl AsRef<Path>) {
        *self = Self::new();
        self.load(addon_path);
    }

    pub fn get(&self, index: &Type) -> Table {
        let lua = self.data.get(index).expect(&format!("Addon content index {:?} does not exist", index));
        lua.globals().get("Block").unwrap()
    }

    fn load_blocks(&mut self, path: impl AsRef<Path>) { // TODO: More verbose errors
        for entry in resources::read_dir(path.as_ref().join("blocks")).unwrap() {
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
            let code = resources::read_dir_entry_string(entry.as_ref().unwrap()).unwrap();
            lua.load(&code).exec().unwrap();

            // Parse output
            let block: Table = lua.globals().get("Block").unwrap();
            let namespace = block.get("namespace").unwrap();
            let id = block.get("id").unwrap();
            drop(block);

            self.data.insert(Type::Block(namespace, id), lua);
        }
    }
}

impl InnerModule<()> for AddonManager {}

struct Addon {
}

impl Addon {
    fn new() -> Self {
        Self {
        }
    }
}