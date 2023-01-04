use super::PlayerActions;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn get_keycode_from_line<'a>(line: &'a String) -> String {
    let mut enumstring: &str = "notfound";
    for (ind, charac) in line.as_bytes().into_iter().enumerate() {
        if charac.clone() as char == ':' {
            //from 0 to :
            enumstring = &line[0..ind];
            break;
        }
    }
    //get_enum_from_str(enumstring)
    enumstring.to_string()
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

pub fn update_key_bindings() -> HashMap<String, PlayerActions> {
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
