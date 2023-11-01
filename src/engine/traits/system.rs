use crate::engine::traits::Entity;

pub trait System {
    fn get_entity(&self, name: &str) -> Result<&dyn Entity, String>;
    fn get_mut_entity(&mut self, name: &str) -> Result<&mut dyn Entity, String>;
    fn add_entity(&mut self, entity: Box<dyn Entity>) -> Result<(), String>;
    fn remove_entity(&mut self, name: &str) -> Result<(), String>;
    fn update(&mut self);
}
