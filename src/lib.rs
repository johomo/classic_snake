pub mod engine;

use std::error;

pub fn run () -> Result<(), Box<dyn error::Error>> {
    Ok(())
}

/*
TUI (Terminal User Interface): crossterm

Board: height, width

Game engine
-----------
System:
    holds entities:
        methods to add/get/remove/update entities
    regular updates:
        get time since last interval (?)
    builtin-components:
        keyboard input, update last key on key press



Snake-classic
-------------
(uses system)
entity:
  - snake
      -  components: ->
      -  keyboard_movement
      -  attributes (size, colour, etc)
  - food
      - components: ->
      - attributes (size, colour, etc)
 */
