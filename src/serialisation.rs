use std::io::Write;

use json::{object, JsonValue};

use crate::game_state::buildings::material::Material;
use crate::game_state::{self, tile::Tile, MainState};
use game_state::buildings::state::State;
use game_state::game_resources::PermanentGameResources;
use game_state::game_resources::MaterialValue;
    //
mod load_utils;

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

    let json_data= match  load_utils::get_json_data_from_file(filename){
        Some(data)=>data,
        None=>return None,
    };

    //list of resourses we are looking for
    let resourse_data_to_fech=PermanentGameResources::get_serialisable_materials_info();

    //vec that we are going to give to the mainstate constructor
    let mut resourse_data_to_send:Vec<(Material,MaterialValue)> =vec![];

    //get resource data from json
    for (material,matval) in resourse_data_to_fech.iter(){
        resourse_data_to_send.push((material.clone(),
        match  load_utils::get_material_value_from_json(material,matval,json_data.clone()){
            Some(asd)=>asd,
            None=>return None,
        } ))
    }


    //load map
    let saved_map = &json_data["Map"];
    //vector of tiles, can give directly to mainstate constructor
    let mut tilevec: Vec<Tile> = vec![];

    
    for i in 0..saved_map.len() {
        //this should be [16,9,"FactoryBlock"]
        
        let savedx= match load_utils::get_cordinate_from_index(saved_map, i,0){
            Some(val)=>val,
            None=>return None,
        };
        let savedy= match load_utils::get_cordinate_from_index(saved_map, i,1){
            Some(val)=>val,
            None=>return None,
        };

        //construct tile and add it to the tilevec
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
