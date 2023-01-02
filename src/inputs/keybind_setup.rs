use super::PlayerActions;
use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

//this can be fixed if we can implement fmt display to keycode
//or some other way to turn this enum to a string or char
fn get_enum_from_str(find: &str) -> KeyCode {
    match find {
        "K" => KeyCode::K,
        "T" => KeyCode::T,
        "Y" => KeyCode::Y,
        "J" => KeyCode::J,
        "L" => KeyCode::L,
        "H" => KeyCode::H,
        "W" => KeyCode::W,
        "A" => KeyCode::A,
        "S" => KeyCode::S,
        "D" => KeyCode::D,
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,

        _ => panic!("key: {} in KeyboardInputActions.cfg is not supported", find),
        // a key that is in KeyboardInputActions.cfg is not defined here
    }
}

fn get_keycode_from_line<'a>(line: &'a String) -> KeyCode {
    let mut enumstring: &str = "notfound";
    for (ind, charac) in line.as_bytes().into_iter().enumerate() {
        if charac.clone() as char == ':' {
            //from 0 to :
            enumstring = &line[0..ind];
            break;
        }
    }
    get_enum_from_str(enumstring)
}

//line should looke like this K:MakeFactory;
//match string[0] with a string version of keycode enum
//do same with our enum
//PlayerActions enum as a string
fn get_playeraction_from_line<'a>(line: &'a String) -> &'a str {
    let mut enumstring: &str = "NoAction";
    for (ind, charac) in line.as_bytes().into_iter().enumerate() {
        if charac.clone() as char == ':' {
            //where the enum starts index of charac
            //we know when it ensd
            enumstring = &line[ind + 1..line.len() - 1];
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

pub fn update_key_bindings() -> HashMap<KeyCode, PlayerActions> {
    let file_result = std::fs::File::open(crate::KEYBIND_FILENAME);
    let mut returnmap = HashMap::new();

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!(
            "opening file: {} Resulted in: {} error ",
            crate::KEYBIND_FILENAME,
            error
        ),
    };

    let reader = BufReader::new(file);
    for ln in reader.lines() {
        let line = match ln {
            Ok(line) => line,
            Err(error) => panic!("{}", error),
        };
        //line should looke like this K:MakeFactory;

        //get PlayerActions enum varian from the line
        let enumstring = get_playeraction_from_line(&line);

        //get keycode from line function same as the upper one but other
        let key = get_keycode_from_line(&line);
        returnmap.insert(key, PlayerActions::get_enum_from_string(enumstring));
        //println!("key:{} val:{}",line.chars().nth(0).unwrap(),enumstring);
    }
    //println!("{}",returnmap.len());
    returnmap
}
