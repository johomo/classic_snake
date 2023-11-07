use crate::engine::component::{traits::Component, AsAny};
use crate::engine::entity::Entity;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Weak;

pub enum Rotation {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rotation::North => write!(f, "North"),
            Rotation::South => write!(f, "South"),
            Rotation::East => write!(f, "East"),
            Rotation::West => write!(f, "West"),
        }
    }
}

pub struct Transform {
    id: String,
    position: [usize; 3],
    rotation: Rotation,
    parent: Weak<RefCell<Entity>>,
}

impl Transform {
    pub fn new(
        id: String,
        position: [usize; 3],
        rotation: Rotation,
        parent: Weak<RefCell<Entity>>,
    ) -> Transform {
        Transform {
            id,
            position,
            rotation,
            parent,
        }
    }
    pub fn get_position(&self) -> &[usize; 3] {
        &self.position
    }
    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }
    pub fn set_position(&mut self, position: [usize; 3]) {
        self.position = position;
    }
    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }
}

impl Component for Transform {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn update(&self) {}
}

impl AsAny for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
