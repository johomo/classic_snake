use crate::engine::traits::{AsAny, Component};
use std::any::Any;
use std::fmt;

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
    name: String,
    position: [usize; 3],
    rotation: Rotation,
}

impl Transform {
    pub fn new(name: String, position: [usize; 3], rotation: Rotation) -> Transform {
        Transform {
            id: String::from("1234"),
            name,
            position,
            rotation,
        }
    }

    pub fn walk(&mut self, units: usize, direction: Rotation) {
        match direction {
            Rotation::North => self.position[0] += units,
            Rotation::South => self.position[0] -= units,
            Rotation::East => self.position[1] += units,
            Rotation::West => self.position[1] -= units,
        }
        self.rotation = direction;
    }

    pub fn get_position(&self) -> &[usize; 3] {
        &self.position
    }
    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }
}

impl Component for Transform {
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn update(&mut self) {}
}

impl AsAny for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
