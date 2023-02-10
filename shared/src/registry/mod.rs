use std::collections::HashMap;

use crate::{
    util::GetOrInsert,
    InnerModule,
};

use self::types::RegistryType;

pub mod types;

pub struct Registry<T: RegistryType> {
    entries: HashMap<String, HashMap<u32, T>>,
}

impl<T: RegistryType> InnerModule<()> for Registry<T> {}

impl<T: RegistryType> Registry<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, entry: T) {
        let (namespace, id) = entry.get_id();

        let entries = self.entries.get_or_insert(namespace, HashMap::new());
        entries.insert(id, entry);
    }

    pub fn get(&self, namespace: impl Into<String>, id: u32) -> &T {
        let namespace = namespace.into();
        &self
            .entries
            .get(&namespace)
            .expect(&format!("Namespace {} not found!", &namespace))
            .get(&id)
            .expect(&format!("Entry {}:{} not found!", &namespace, id))
    }
}