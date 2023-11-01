use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::engine::traits;
pub struct System {
    entities: HashMap<String, Box<dyn traits::Entity>>,
}
impl traits::System for System {
    fn get_entity(&self, name: &str) -> Result<&dyn traits::Entity, String> {
        match self.entities.get(name) {
            Some(value) => Ok(value.as_ref()),
            None => Err(format!(
                "System does not have any entity with name {}",
                name
            )),
        }
    }
    fn get_mut_entity(&mut self, name: &str) -> Result<&mut dyn traits::Entity, String> {
        match self.entities.get_mut(name) {
            Some(value) => Ok(value.as_mut()),
            None => Err(format!(
                "System does not have any entity with name {}",
                name
            )),
        }
    }
    fn add_entity(&mut self, entity: Box<dyn traits::Entity>) -> Result<(), String> {
        let name = entity.get_name();
        match self.entities.entry(name.to_string()) {
            Entry::Occupied(_) => Err(format!(
                "Cannot add entity with name {}. An entity with the same name already exists.",
                name
            )),
            Entry::Vacant(v) => {
                v.insert(entity);
                Ok(())
            }
        }
    }
    fn remove_entity(&mut self, name: &str) -> Result<(), String> {
        match self.entities.entry(name.to_string()) {
            Entry::Occupied(o) => {
                o.remove();
                Ok(())
            }
            Entry::Vacant(_) => Err(format!(
                "Cannot remove entity with name {}. Entity does not exist.",
                name
            )),
        }
    }
    fn update(&mut self) {
        self.entities
            .values_mut()
            .map(|entity| entity.update())
            .last();
    }
}
