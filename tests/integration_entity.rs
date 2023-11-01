use classic_snake::engine::component::{Rotation, Transform};
use classic_snake::engine::traits;
use classic_snake::engine::Entity;

#[test]
fn get_id() {
    let entity: Box<dyn traits::Entity> = Box::new(Entity::new(String::from("snake")));
    let expected = "snake";
    let actual = entity.get_name();
    assert_eq!(expected, actual);
}

#[test]
fn add_get_remove_component() -> Result<(), String> {
    // Setup
    let mut entity: Box<dyn traits::Entity> = Box::new(Entity::new(String::from("snake")));
    let position: Box<dyn traits::Component> = Box::new(Transform::new(
        String::from("head"),
        [2, 3, 0],
        Rotation::North,
    ));

    // Add component
    entity.add_component(position)?;

    // Get component
    let component = entity.get_component("head")?;
    let expected = "head";
    let actual = component.get_name();
    if expected != actual {
        return Err(format!(
            "Expected component with name {}, but got name {}",
            expected, actual
        ));
    }

    // Remove component
    entity.remove_component("head")?;

    // Get component
    let component = entity.get_component("head");
    if component.is_ok() {
        let name = component.unwrap().get_name();
        return Err(format!(
            "Expected no component, but got component with name {}",
            name
        ));
    }
    Ok(())
}

#[test]
fn add_get_mut_mutate_component() -> Result<(), String> {
    // Setup
    let mut entity: Box<dyn traits::Entity> = Box::new(Entity::new(String::from("snake")));
    let position: Box<dyn traits::Component> = Box::new(Transform::new(
        String::from("head"),
        [2, 3, 0],
        Rotation::North,
    ));
    entity.add_component(position)?;

    // Get mut component
    let component = entity.get_component_mut("head")?;
    let expected = "head";
    let actual = component.get_name();
    if expected != actual {
        return Err(format!(
            "Expected component with name {}, but got name {}",
            expected, actual
        ));
    }

    // Mutate component
    let transform = component.as_any_mut().downcast_mut::<Transform>().unwrap();
    transform.walk(3, Rotation::West);

    // Get component
    let component = entity.get_component("head")?;
    let transform = component.as_any().downcast_ref::<Transform>().unwrap();

    // Assertion
    let expected_position = [2, 0, 0];
    let actual_position = *transform.get_position();
    let expected_rotation = Rotation::West;
    let actual_rotation = transform.get_rotation();
    if expected_position != actual_position {
        return Err(format!(
            "Expected component with position {:?}, but got {:?}",
            expected_position, actual_position,
        ));
    }
    match actual_rotation {
        Rotation::West => Ok(()),
        _ => Err(format!(
            "Expected component with rotation {}, but got {}",
            expected_rotation, actual_rotation,
        )),
    }?;
    Ok(())
}
