use json::JsonValue;
use json::array;
use json::object;

use super::buildings::material::Material;
use super::buildings::material::PermanentMaterial;
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
    

    pub fn make_from_vec(save:Vec<(PermanentMaterial,MaterialValue)>)->Self{
        let mut retval:PermanentGameResources=Self { money: 1 };

        for (mat,i) in save{
            match mat {
                PermanentMaterial::Money=>retval.add_to_resource(i, mat),
            }
        }
        retval

    }


    //needs to be updated for every added resource
    pub fn get_serialisable_materials_info()->Vec<(PermanentMaterial,MaterialValue)>{
        let mut retvec=vec![];
        retvec.push( (PermanentMaterial::Money, MaterialValue::I32(0)) );

        retvec
    }


    //needs to be updated for every added resource
    pub fn get_as_serialisable(&self)->JsonValue{
        let mut returned_json = json::JsonValue::new_object();
        returned_json[PermanentMaterial::Money.to_string()]=self.money.into();

        returned_json
    }


    pub fn get_money(&self)->&i32{
        &self.money
    }

    //reformat
    pub fn add_to_resource(&mut self,val:MaterialValue,mat:PermanentMaterial){
        match mat {
            PermanentMaterial::Money=>{
                let added_val:i32=match val {
                    MaterialValue::I32(val)=>val,
                    _=>panic!("tried to add wrong thing to money"),
                };
                self.money+=added_val;
            },

        }

    }





}




pub struct GameResources {
    perm_resources:PermanentGameResources,
    temp_money: i32,
    temp_money_multiplier: f32,
    last_collection_income: i32,
}

impl GameResources {
    pub fn make_instance() -> Self {
        GameResources {
            perm_resources:PermanentGameResources::create_empty(),
            temp_money: 0,
            temp_money_multiplier: 1.0,
            last_collection_income: 0,
        }
    }

    //this needs to change if we want to make it scalable, cannot change both of these every time
    //mayby make saved res its own thing

    pub fn make_instance_with_permanent(res:PermanentGameResources)->Self{
        GameResources { perm_resources: res, temp_money: 0, temp_money_multiplier: 1.0, last_collection_income: 0 }
    }



    fn get_total_collection_money(&self) -> i32 {
        (self.temp_money as f32 * self.temp_money_multiplier) as i32
    }

    //mess
    pub fn collapse_money(&mut self) {
        self.last_collection_income = 0;
        self.last_collection_income += self.get_total_collection_money();
        self.perm_resources.add_to_resource(MaterialValue::I32(self.last_collection_income), PermanentMaterial::Money);
        self.temp_money = 0;
        self.temp_money_multiplier = 1.0;
    }

    pub fn add_resource(&mut self, building_info: Building) {
        match building_info.created_material {
            Material::Money => self.add_money(building_info.produced_amount as i32),
            Material::MoneyMultiplier => self.add_multiplier(building_info.produced_amount),
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
        self.perm_resources.add_to_resource(MaterialValue::I32(-value), PermanentMaterial::Money) ;
    }

    pub fn add_multiplier(&mut self, val: f32) {
        self.temp_money_multiplier += val;
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
