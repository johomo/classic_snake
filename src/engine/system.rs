use crate::engine::Entity;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct System {
    entities: HashMap<String, Rc<RefCell<Entity>>>,
    self_weak: Weak<RefCell<Self>>,
}

impl System {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(System {
                entities: HashMap::new(),
                self_weak: weak.clone(),
            })
        })
    }

    pub fn new_entity(&mut self, id: String) -> Result<(), String> {
        match self.entities.entry(id) {
            Entry::Occupied(o) => Err(format!(
                "Cannot create entity with id {}. An entity with the same id already exists.",
                o.key()
            )),
            Entry::Vacant(v) => {
                let entity = Entity::new(v.key().to_string(), self.self_weak.clone());
                v.insert(entity);
                Ok(())
            }
        }
    }

    pub fn get_entity(&self, id: &str) -> Result<Ref<Entity>, String> {
        match self.entities.get(id) {
            Some(value) => Ok(value.borrow()),
            None => Err(format!("System does not have any entity with id {}", id)),
        }
    }

    pub fn get_mut_entity(&self, id: &str) -> Result<RefMut<Entity>, String> {
        match self.entities.get(id) {
            Some(value) => Ok(value.borrow_mut()),
            None => Err(format!("System does not have any entity with id {}", id)),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), String> {
        let id = entity.get_id().to_string();
        match self.entities.entry(id) {
            Entry::Occupied(o) => Err(format!(
                "Cannot add entity with id {}. An entity with the same id already exists.",
                o.key(),
            )),
            Entry::Vacant(v) => {
                v.insert(Rc::new(RefCell::new(entity)));
                Ok(())
            }
        }
    }

    pub fn remove_entity(&mut self, id: &str) -> Result<(), String> {
        match self.entities.entry(id.to_string()) {
            Entry::Occupied(o) => {
                o.remove();
                Ok(())
            }
            Entry::Vacant(v) => Err(format!(
                "Cannot remove entity with id {}. Entity does not exist.",
                v.key()
            )),
        }
    }

    pub fn update(&self) {
        self.entities
            .values()
            .map(|entity| entity.borrow_mut().update())
            .last();
    }
}
