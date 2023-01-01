use std::collections::HashMap;
use std::io::{BufReader, BufRead};

use ggez;
use ggez::input::keyboard::KeyCode;
use super::game_state::tile_state::State;
use super::MainState;



pub mod player_actions;
pub mod keyboard_input_data;
use player_actions::PlayerActions;


//todo implement panning
//todo make this not shit
//
//
//public interfaces
//update keybindings
//handle KeyboardInputActions
//there are orphan functions in this file. do something about them

//all actions the player can do




//this can be fixed if we can implement fmt display to keycode
//or some other way to turn this enum to a string or char
fn get_enum_from_char(find:char)->KeyCode{
    match find {
        'K'=>KeyCode::K,
        'J'=>KeyCode::J,
        'L'=>KeyCode::L,
        'H'=>KeyCode::H,
        'W'=>KeyCode::W,
        'A'=>KeyCode::A,
        'S'=>KeyCode::S,
        'D'=>KeyCode::D,
        _=>panic!("no key: {} implemented",find),
        
    }
}


//line should looke like this K:MakeFactory;
//match string[0] with a string version of keycode enum
//do same with our enum
//PlayerActions enum as a string
fn get_playeraction_from_line<'a>(line:&'a String)->&'a str{
    let mut enumstring:&str="NoAction";
    for (ind,charac) in line.as_bytes().into_iter().enumerate(){
        if charac.clone() as char==':'{
            //where the enum starts index of charac
            //we know when it ensd
            enumstring = &line[ind+1..line.len()-1];
            break;
        }
    }
    enumstring
}





//reads the keyboard.cfg file and updates the dictionary in mainstate that containts all the
//keycode, playeracrion
//
//do better error handling
//rewrite the entire funktion and break in into multiple ones

//things this function does,
//opens file. 
//goes through every line
//goes through every letter
//first letter is keycode enum
//if letter is : then the rest is enum playeracrion
//append keycode,playeracrion to a hashmap
//
//return hashmap
//

//we can make a get enum(playeracrion) from line function
//this removes 7 lines of code


pub fn update_key_bindings()->HashMap<KeyCode,PlayerActions>{
    let file_result=std::fs::File::open(crate::KEYBIND_FILENAME);
    let mut returnmap=HashMap::new();

    let file=match file_result{
        Ok(file)=>file,
        Err(error)=>panic!("opening file: {} Resulted in: {} error ",crate::KEYBIND_FILENAME,error),
    };
    
    let reader = BufReader::new(file);
    for ln in reader.lines(){
        let line =match ln{
            Ok(line)=>line,
            Err(error)=>panic!("{}",error),
        };

        //line should looke like this K:MakeFactory;
        //get PlayerActions enum varian from the line
        let enumstring=get_playeraction_from_line(&line);

        let key=get_enum_from_char(line.chars().nth(0).unwrap());
        returnmap.insert(key, PlayerActions::get_enum_from_string(enumstring));
        //println!("key:{} val:{}",line.chars().nth(0).unwrap(),enumstring);
    };
    //println!("{}",returnmap.len());
    returnmap

}


//finds key from the hash keybindings and activates its effect if there is one
fn activate_key(key:&KeyCode,game:&mut MainState){
    let action =game.get_key_map().get(key);
    match action {
        Some(action)=>action.apply_effect(game),
        None=>()//no key found in keymap
    }
}





//loops all keys and activates its effect if it exists
pub fn handle_keyboard_inputs(game: &mut MainState, ctx: &mut ggez::Context) {

    let currently_pressed_keys=ctx.keyboard.pressed_keys();
    //loop through all currently pressed keys and see if we have allready handled them
    for key in currently_pressed_keys{
        if !game.get_input_data().is_key_handled(key){
            activate_key(key, game);
            game.get_mut_input_data().handled_keys.push(key.clone());
        }
    }

    //remove handled key if it is not in currently_pressed_keys
    //this way we can have a debounce
    let mut keys_to_remove_from_handled:Vec<KeyCode>=vec![];
    for handled_key in game.get_input_data().handled_keys.iter(){
        let mut is_in_pressed=false;

        for key in currently_pressed_keys{
            if handled_key==key{
                is_in_pressed=true;
            }
        }
        if !is_in_pressed{
            keys_to_remove_from_handled.push(handled_key.clone())
        }
    }

    //seacond loop so that the first mut ref of game goes out of scope
    //remove keys from handled_keys
    for i in keys_to_remove_from_handled.iter(){
        game.get_mut_input_data().handled_keys.retain(|value| *value != i.clone());
    }

}
