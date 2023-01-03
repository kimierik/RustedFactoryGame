use super::player_actions;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

//make getsetters
pub struct InputData {
    //change enum keycode to string so we can do stuff better
    pub key_map: HashMap<String, player_actions::PlayerActions>,
    pub handled_keys: Vec<ggez::input::keyboard::KeyCode>,
}

impl InputData {
    pub fn new() -> Self {
        InputData {
            key_map: super::keybind_setup::update_key_bindings(),
            handled_keys: vec![],
        }
    }

    pub fn is_key_handled(&self, find: &KeyCode) -> bool {
        for i in self.handled_keys.iter() {
            if i == find {
                return true;
            }
        }
        false
    }
}
