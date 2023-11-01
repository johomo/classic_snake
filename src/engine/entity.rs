use crate::engine::traits;
use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Entity {
    id: String,
    name: String,
    components: HashMap<String, Box<dyn traits::Component>>,
}
impl Entity {
    pub fn new(name: String) -> Entity {
        Entity {
            id: String::from("1234"),
            name,
            components: HashMap::new(),
        }
    }
}

impl traits::Entity for Entity {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_component(&self, name: &str) -> Result<&dyn traits::Component, String> {
        match self.components.get(name) {
            Some(value) => Ok(value.as_ref()),
            None => Err(format!(
                "Entity {} does not have any component with name {}",
                self.id, name
            )),
        }
    }
    fn get_component_mut(&mut self, name: &str) -> Result<&mut dyn traits::Component, String> {
        match self.components.get_mut(name) {
            Some(value) => Ok(value.as_mut()),
            None => Err(format!(
                "Entity {} does not have any component with name {}",
                self.id, name
            )),
        }
    }
    fn add_component(&mut self, component: Box<dyn traits::Component>) -> Result<(), String> {
        let name = component.get_name();
        match self.components.entry(name.to_string()) {
            Entry::Occupied(_) => Err(
                format!("Cannot add component with name {} into entity with name {}. A component with the same name already exists.", name, self.name)
            ),
            Entry::Vacant(v) => {
                v.insert(component);
                Ok(())
            }
        }
    }
    fn remove_component(&mut self, name: &str) -> Result<(), String> {
        match self.components.entry(name.to_string()) {
            Entry::Occupied(o) => {
                o.remove();
                Ok(())
            },
            Entry::Vacant(_) => Err(
                format!("Cannot remove component with name {} from entity with name {}. Component does not exist.", name, self.name)
            )
        }
    }
    fn update(&mut self) {
        self.components
            .values_mut()
            .map(|component| component.update())
            .last();
    }
}

impl traits::AsAny for Entity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
