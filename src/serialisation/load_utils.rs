
use std::io::Read;
use json::JsonValue;
use crate::game_state::buildings::material::PermanentMaterial;
use super::game_state::game_resources::MaterialValue;


//file is to house functions load game uses in a seperate file



//open file and return JsonValue from it
pub fn get_json_data_from_file(filename:&str)->Option< JsonValue>{

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
    Some(json_data)
}




pub fn get_cordinate_from_index(saved_map:&JsonValue, i:usize,index:usize)->Option<f32>{
        let returnnum: f32 = match saved_map[i][index].as_f32() {
            Some(num) => num,
            None => {
                println!(
                    "Load failed: tile cordinate {} cannot be converted to num",
                    saved_map[i][index]
                );
                return None;
            }
        };
        Some(returnnum)
}



pub fn get_material_value_from_json(material:&PermanentMaterial,matval:&MaterialValue,json_data:JsonValue)->Option< MaterialValue>{
    match matval {
        MaterialValue::I32(_num)=>{
            println!("{}",json_data["Resources"][material.to_string()]);
            println!("{}",material);
            let saved_money = match json_data["Resources"][material.to_string()].as_i32() {
                Some(number) => number,
                None => {
                    println!("error converting money ");
                    return None;
                }
            };
            Some(MaterialValue::I32(saved_money))
        },

        MaterialValue::F32(_num)=>{
            let saved_money = match json_data["Resources"][material.to_string()].as_f32() {
                Some(number) => number,
                None => {
                    println!("error converting money ");
                    return None;
                }
            };
            Some( MaterialValue::F32(saved_money))
        },
    }

}
