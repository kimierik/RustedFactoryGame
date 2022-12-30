use core::fmt;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

use ggez;
use ggez::input::keyboard::KeyCode;
use super::game_state::tile_state::State;
use super::MainState;



pub mod player_actions;
use player_actions::PlayerActions;


//todo implement panning
//todo make this not shit
//
//
//public interfaces
//update keybindings
//handle KeyboardInputActions

//all actions the player can do




//this can be fixed if we can implement fmt to keycode
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
        //match string[0] with a string version of keycode enum
        //do same with our enum
        let mut enumstring:&str="NoAction";

        //?
        for (ind,charac) in line.as_bytes().into_iter().enumerate(){
            if charac.clone() as char==':'{
                //where the enum starts index of charac
                //we know when it ensd
                enumstring = &line[ind+1..line.len()-1];
                break;
            }
        }

        //get the proper enum
        //find where the string is represented in the line
        //match it to the one in the enum

        let key=get_enum_from_char(line.chars().nth(0).unwrap());
        returnmap.insert(key, PlayerActions::get_enum_from_string(enumstring));
        //println!("key:{} val:{}",line.chars().nth(0).unwrap(),enumstring);
    };
    //println!("{}",returnmap.len());
    returnmap

}




//idk if small irersponsiveness but fix on remake
//prob just no money no irresponsiveness
//this can be fixed to loop every key that is pressed then apply_effect
//
//this would need some way to turn a "keycode" into enum keycode 
//or not just some no key value to be returned that returns noacton always
pub fn handle_keyboard_inputs(game: &mut MainState, ctx: &mut ggez::Context) {
    if ctx.keyboard.is_key_just_pressed(KeyCode::D) {
        game.get_key_map().get(&KeyCode::D).unwrap().apply_effect(game);
    }

    if ctx.keyboard.is_key_just_pressed(KeyCode::A) {
        game.get_key_map().get(&KeyCode::A).unwrap().apply_effect(game);
    }

    if ctx.keyboard.is_key_just_pressed(KeyCode::W) {
        game.get_key_map().get(&KeyCode::W).unwrap().apply_effect(game);
    }

    if ctx.keyboard.is_key_just_pressed(KeyCode::S) {
        game.get_key_map().get(&KeyCode::S).unwrap().apply_effect(game);
    }

    //these can be changed to is key pressd if i want to paint
    if ctx.keyboard.is_key_just_pressed(KeyCode::L) {
        game.get_key_map().get(&KeyCode::L).unwrap().apply_effect(game);
    }

    if ctx.keyboard.is_key_just_pressed(KeyCode::K) {
        game.get_key_map().get(&KeyCode::K).unwrap().apply_effect(game);
    }
}
