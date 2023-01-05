use json::JsonValue;

use super::buildings::material::Material;
use super::buildings::material::BuildingType;
use super::buildings::Building;


//brake bile into multiple files


//a simple way to possibly have i32 and f32 values that can be used as parameters
#[allow(dead_code)]
pub enum MaterialValue{
    I32(i32),
    F32(f32),
}



pub struct PermanentGameResources{
    money:i32,
}

impl PermanentGameResources{
    
    pub fn create_empty()->Self{
        PermanentGameResources { money: 1 }
    }
    

    pub fn make_from_vec(save:Vec<(Material,MaterialValue)>)->Self{
        let mut retval:PermanentGameResources=Self { money: 1 };

        for (mat,i) in save{
            match mat {
                Material::Money=>retval.add_to_resource(i, mat),
            }
        }
        retval

    }


    //needs to be updated for every added resource
    pub fn get_serialisable_materials_info()->Vec<(Material,MaterialValue)>{
        let mut retvec=vec![];
        retvec.push( (Material::Money, MaterialValue::I32(0)) );

        retvec
    }


    //needs to be updated for every added resource
    pub fn get_as_serialisable(&self)->JsonValue{
        let mut returned_json = json::JsonValue::new_object();
        returned_json[Material::Money.to_string()]=self.money.into();

        returned_json
    }


    pub fn get_money(&self)->&i32{
        &self.money
    }

    //reformat
    pub fn add_to_resource(&mut self,val:MaterialValue,mat:Material){
        match mat {
            Material::Money=>{
                let added_val:i32=match val {
                    MaterialValue::I32(val)=>val,

                    _=>panic!("tried to add wrong thing to money"),
                };
                self.money+=added_val;
                //println!("{}",added_val);
            },

        }

    }





}




pub struct GameResources {
    perm_resources:PermanentGameResources,
    temp_money: i32,
    last_collection_income: i32,
}

impl GameResources {
    pub fn make_instance() -> Self {
        GameResources {
            perm_resources:PermanentGameResources::create_empty(),
            temp_money: 0,
            last_collection_income: 0,
        }
    }

    //this needs to change if we want to make it scalable, cannot change both of these every time
    //mayby make saved res its own thing

    pub fn make_instance_with_permanent(res:PermanentGameResources)->Self{
        GameResources { perm_resources: res, temp_money: 0,  last_collection_income: 0 }
    }


    //mess
    pub fn collapse_money(&mut self) {
        self.last_collection_income=self.temp_money;
        self.perm_resources.add_to_resource(MaterialValue::I32(self.temp_money), Material::Money);
        self.temp_money = 0;
    }

    //gets called tiwce
    pub fn add_resource(&mut self, building_info: &Building,multiplier:f32) {
        match &building_info.building_type {
            BuildingType::Production(mat)=>  match  mat{
                Material::Money => self.add_money((building_info.produced_amount * multiplier) as i32),
            },

            BuildingType::Buff(_)=>(),
        }
    }

    pub fn get_permanent_resources(&self)->&PermanentGameResources{
        &self.perm_resources
    }

    //add money to temp money
    pub fn add_money(&mut self, value: i32) {
        self.temp_money += value;
    }

    //permanent money subtract
    pub fn subtract_money(&mut self, value: i32) {
        self.perm_resources.add_to_resource(MaterialValue::I32(-value), Material::Money) ;
    }


    pub fn get_money(&self) -> &i32 {
        &self.perm_resources.get_money()
    }

    pub fn to_string(&self) -> String {
        format!(
            "money: {} \ncurrent income: {}",
            self.perm_resources.get_money(), self.last_collection_income
        )
    }
}
