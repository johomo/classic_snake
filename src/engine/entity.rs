use crate::engine::component::{AsAny, Component};
use crate::engine::component::{Rotation, Transform};
use crate::engine::system::System;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct Entity {
    id: String,
    components: HashMap<String, Rc<RefCell<dyn Component>>>,
    system: Weak<RefCell<System>>,
    self_weak: Weak<RefCell<Self>>,
}

impl Entity {
    pub fn new(id: String, system: Weak<RefCell<System>>) -> Rc<RefCell<Entity>> {
        Rc::new_cyclic(|weak| {
            let mut entity = Entity {
                id,
                components: HashMap::new(),
                system,
                self_weak: weak.clone(),
            };
            let transform = Transform::new(
                String::from("transform"),
                [0, 0, 0],
                Rotation::North,
                entity.self_weak.clone(),
            );
            entity
                .add_component(Rc::new(RefCell::new(transform)))
                .unwrap();
            RefCell::new(entity)
        })
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_component(&self, id: &str) -> Result<Ref<dyn Component>, String> {
        match self.components.get(id) {
            Some(value) => Ok(value.borrow()),
            None => Err(format!(
                "Entity {} does not have any component with id {}",
                self.id, id
            )),
        }
    }

    pub fn get_component_mut(&self, id: &str) -> Result<RefMut<dyn Component>, String> {
        match self.components.get(id) {
            Some(value) => Ok(value.borrow_mut()),
            None => Err(format!(
                "Entity {} does not have any component with id {}",
                self.id, id
            )),
        }
    }

    pub fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) -> Result<(), String> {
        let id = component.borrow().get_id().to_string();
        match self.components.entry(id) {
            Entry::Occupied(o) => Err(
                format!("Cannot add component with id {} into entity with id {}. A component with the same id already exists.", o.key() , self.id)
            ),
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            }
        }
    }

    pub fn remove_component(&mut self, id: String) -> Result<(), String> {
        match self.components.entry(id) {
            Entry::Occupied(o) => {
                o.remove();
                Ok(())
            },
            Entry::Vacant(v) => Err(
                format!("Cannot remove component with id {} from entity with id {}. Component does not exist.", v.key(), self.id)
            )
        }
    }

    pub fn update(&self) {
        self.components
            .values()
            .map(|component| component.borrow_mut().update())
            .last();
    }
}

impl AsAny for Entity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
