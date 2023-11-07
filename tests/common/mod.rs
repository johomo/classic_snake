use classic_snake::engine::System;
use std::cell::RefCell;
use std::rc::Rc;

pub fn new_system() -> Rc<RefCell<System>> {
    let system = System::new();
    system
        .borrow_mut()
        .new_entity(String::from("head"))
        .unwrap();
    return system;
}
