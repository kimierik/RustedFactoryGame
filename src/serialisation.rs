
use std::io::{Write, Read};

use json::{object, JsonValue};

use crate::game_state::{MainState, tile_state::{self, State}, self, tile::Tile};

pub enum GameOptions{
    NewGame,
    LoadGame(String),
}


pub fn save_game(game:&MainState){

    let mut mapthing:Vec<JsonValue>=vec![];
    for i in game.get_map().iter(){
        mapthing.push(i.get_as_serialisable());
    }
    let mut serialised_array=json::array!{};
    for i in mapthing{
        serialised_array.push(i).unwrap();
    }

    let money=game.get_resource().get_money().clone();
    let save=object! {
        Resources:{ Money:money },
        Map:serialised_array,
    };
    //then write save

    let file_result = std::fs::File::create("test_save.json");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => panic!(
            "opening file: {} Resulted in: {} error ",
            "test_save",
            error
        ),
    };
    
    match file.write_all(save.dump().as_bytes()){
        Ok(ok)=>ok,
        Err(err)=>println!("{:?}",err),
    };

}


//reads file and makes 
pub fn load_game(filename:&str)->MainState{
    //TODO
    //make better error messeges
    //remove unwraps
    let mut file =std::fs::File::open(filename).unwrap();
    let mut s:String=String::new();
    file.read_to_string(&mut s).unwrap();
    let b=json::parse(&s).unwrap();
    let saved_money=&b["Resources"]["Money"];
    let saved_map=&b["Map"];
    //println!("{}",saved_money);
    //println!("{}",saved_map);

    let mut tilevec:Vec<Tile>=vec![];
    for i in 0..saved_map.len(){
        //println!("{}",saved_map[i]);
        //this is [16,9,"FactoryBlock"]  someway to parse this propper
        //just use indexes as parameters
        //
        let savedx:f32=saved_map[i][0].as_f32().unwrap();
        let savedy:f32=saved_map[i][1].as_f32().unwrap();
        tilevec.push(Tile::create_tile_with(game_state::cordinate::Cordinates 
                { 
                    x: savedx, 
                    y: savedy 
                },
                State::get_enum_from_string(&saved_map[i][2].as_str().unwrap()) ))
    }
    



    MainState::new_from_save(tilevec,saved_money.as_i32().unwrap())
}


