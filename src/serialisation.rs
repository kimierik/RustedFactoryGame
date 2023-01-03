
use std::io::{Write, Read};

use json::{object, JsonValue};

use crate::game_state::{MainState, tile_state::State, self, tile::Tile};

pub enum GameOptions{
    NewGame,
    LoadGame(String),
}

//make not panic on failsave
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


//reads file and makes a mainstate instanse from the files data
pub fn load_game(filename:&str)->Option<MainState> {

    let mut file =match std::fs::File::open(filename){
        Ok(file)=>file,
        Err(err)=>{println!("file open error: {}",err);return None},
    };
    let mut s:String=String::new();

    match file.read_to_string(&mut s)  {
        Ok(a)=>a,
        Err(err)=>{println!("error reading from file {}",err);return None},
    } ;

    let json_data:JsonValue=match json::parse(&s) {
        Ok(data)=>data,
        Err(err)=>{println!("failed to parse json data: {}",err);return None},
    }; 

    let saved_money=match json_data["Resources"]["Money"].as_i32() {
        Some(number)=>number,
        None=>{println!("error converting money ");return None},
    }; 


    let saved_map=&json_data["Map"];
    let mut tilevec:Vec<Tile>=vec![];

    for i in 0..saved_map.len(){
        //this should be [16,9,"FactoryBlock"]  

        let savedx:f32=match saved_map[i][0].as_f32(){
            Some(num)=>num,
            None=>{println!("Load failed: tile cordinate x:{} cannot be converted to num",saved_map[i][0]);return None},
        }; 

        let savedy:f32=match saved_map[i][1].as_f32(){
            Some(num)=>num,
            None=>{println!("Load failed: tile cordinate y:{} cannot be converted to num",saved_map[i][1]);return None},
        }; 

        tilevec.push(Tile::create_tile_with(game_state::cordinate::Cordinates 
                { 
                    x: savedx, 
                    y: savedy 
                },
                State::get_enum_from_string(
                    match saved_map[i][2].as_str() {
                        Some(ok)=>&ok,
                        None=>{println!("error converting tile state to a building");return None},
                    } )
                ))
    }


    Some(MainState::new_from_save(tilevec,saved_money))
}


