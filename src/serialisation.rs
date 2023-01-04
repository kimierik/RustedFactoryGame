use std::io::{Read, Write};

use json::{object, JsonValue};

use crate::game_state::buildings::material::PermanentMaterial;
use crate::game_state::{self, tile::Tile, MainState};
use game_state::buildings::state::State;
use game_state::game_resources::PermanentGameResources;
use game_state::game_resources::MaterialValue;
    //

pub enum GameOptions {
    NewGame,
    LoadGame(String),
}

//make not panic on failsave
pub fn save_game(game: &MainState) {

    let mut mapthing: Vec<JsonValue> = vec![];

    for i in game.get_map().iter() {
        mapthing.push(i.get_as_serialisable());
    }
    
    let mut serialised_array = json::array! {};
    for i in mapthing {
        serialised_array.push(i).unwrap();
    }


    let resources=game.get_resource().get_permanent_resources().get_as_serialisable();


    let save = object! {
        Resources:resources,
        Map:serialised_array,
    };

    //then write save

    let file_result = std::fs::File::create("test_save.json");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => panic!(
            "opening file: {} Resulted in: {} error ",
            "test_save", error
        ),
    };

    match file.write_all(save.dump().as_bytes()) {
        Ok(ok) => ok,
        Err(err) => println!("{:?}", err),
    };
}





//PermanentMaterial from vec
//Permanentressources from permat
//

//reads file and makes a mainstate instanse from the files data
pub fn load_game(filename: &str) -> Option<MainState> {
    let mut file = match std::fs::File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("file open error: {}", err);
            return None;
        }
    };

    let mut s: String = String::new();

    match file.read_to_string(&mut s) {
        Ok(a) => a,
        Err(err) => {
            println!("error reading from file {}", err);
            return None;
        }
    };

    let json_data: JsonValue = match json::parse(&s) {
        Ok(data) => data,
        Err(err) => {
            println!("failed to parse json data: {}", err);
            return None;
        }
    };



    //so we can get an vec of touples from PermanentGameResources that cointains (PermanentMaterial:MaterialValue) val can be-
    //f32 or i32 or whatever we are storing
    //
    //loop resnames save it to something a struct or another vec of touples
    //somethign that we can give to the constructor


    //break this forloop into its own function
    let resourse_data_to_fech=PermanentGameResources::get_serialisable_materials_info();
    let mut resourse_data_to_send:Vec<(PermanentMaterial,MaterialValue)> =vec![];

    for (material,matval) in resourse_data_to_fech.iter(){
        resourse_data_to_send.push((material.clone(),
        match matval {
            MaterialValue::I32(_num)=>{
                let saved_money = match json_data["Resources"][material.to_string()].as_i32() {
                    Some(number) => number,
                    None => {
                        println!("error converting money ");
                        return None;
                    }
                };
                MaterialValue::I32(saved_money)
            },

            MaterialValue::F32(_num)=>{
                let saved_money = match json_data["Resources"][material.to_string()].as_f32() {
                    Some(number) => number,
                    None => {
                        println!("error converting money ");
                        return None;
                    }
                };
                MaterialValue::F32(saved_money)
            },
        }))
    }



    let saved_map = &json_data["Map"];
    let mut tilevec: Vec<Tile> = vec![];

    for i in 0..saved_map.len() {
        //this should be [16,9,"FactoryBlock"]

        let savedx: f32 = match saved_map[i][0].as_f32() {
            Some(num) => num,
            None => {
                println!(
                    "Load failed: tile cordinate x:{} cannot be converted to num",
                    saved_map[i][0]
                );
                return None;
            }
        };

        let savedy: f32 = match saved_map[i][1].as_f32() {
            Some(num) => num,
            None => {
                println!(
                    "Load failed: tile cordinate y:{} cannot be converted to num",
                    saved_map[i][1]
                );
                return None;
            }
        };

        tilevec.push(Tile::create_tile_with(
            game_state::cordinate::Cordinates {
                x: savedx,
                y: savedy,
            },
            State::get_enum_from_string(match saved_map[i][2].as_str() {
                Some(ok) => &ok,
                None => {
                    println!("error converting tile state to a building");
                    return None;
                }
            }),
        ))
    }

    Some(MainState::new_from_save(tilevec, PermanentGameResources::make_from_vec(resourse_data_to_send)))
}
