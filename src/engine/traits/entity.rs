use crate::engine::traits::component::Component;

pub trait Entity {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_component(&self, name: &str) -> Result<&dyn Component, String>;
    fn get_component_mut(&mut self, name: &str) -> Result<&mut dyn Component, String>;
    fn add_component(&mut self, component: Box<dyn Component>) -> Result<(), String>;
    fn remove_component(&mut self, name: &str) -> Result<(), String>;
    fn update(&mut self);
}