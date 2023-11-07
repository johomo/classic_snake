mod common;

use classic_snake::engine::component::{Rotation, Transform};

#[test]
fn new_entity_has_transform_by_default() -> Result<(), String> {
    let system = common::new_system();
    let system_binding = system.borrow();
    let entity = system_binding.get_entity("head")?;
    let component_binding = entity.get_component_mut("transform")?;
    let transform = component_binding
        .as_any()
        .downcast_ref::<Transform>()
        .unwrap();

    let expected_position: &[usize; 3] = &[0, 0, 0];
    let actual_position = transform.get_position();

    if expected_position != actual_position {
        return Err(format!(
            "Expected position {:?}, but got {:?}",
            expected_position, actual_position
        ));
    }

    let expected_rotation = &Rotation::North;
    let actual_rotation = transform.get_rotation();
    match actual_rotation {
        Rotation::North => Ok(()),
        _ => Err(format!(
            "Expected rotation {}, but got {}",
            expected_rotation, actual_rotation,
        )),
    }?;
    Ok(())
}
