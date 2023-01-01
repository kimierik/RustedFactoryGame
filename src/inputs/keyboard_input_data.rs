
use super::player_actions;
use std::collections::HashMap;
use ggez::input::keyboard::KeyCode;

//make getsetters 
pub struct InputData{
    pub key_map:HashMap<ggez::input::keyboard::KeyCode,player_actions::PlayerActions>,
    pub handled_keys :Vec<ggez::input::keyboard::KeyCode>,
}


impl InputData{
    pub fn new()->Self{
        InputData{
            key_map:super::update_key_bindings(),
            handled_keys: vec![],
        }
    }


    pub fn is_key_handled(&self,find:&KeyCode)->bool{
        for i in self.handled_keys.iter(){
            if i==find{
                return true;
            }
        }
        false
    }


}


