use json::JsonValue;

use super::buildings::material::Material;
use super::buildings::material::MaterialValue;
use super::buildings::BuildingType;
use super::buildings::Building;


//brake file into multiple files





pub struct PermanentGameResources{
    money:i32,
    rock:i32,
}

impl PermanentGameResources{
    
    pub fn create_empty()->Self{
        PermanentGameResources { money: 1,rock:0 }
    }
    

    pub fn make_from_vec(save:Vec<(Material,MaterialValue)>)->Self{
        let mut retval:PermanentGameResources=Self { money: 1,rock:0 };

        for (mat,i) in save{
            match mat {
                Material::Money=>retval.add_to_resource(i, mat),
                Material::Rock=>retval.add_to_resource(i, mat),
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
    pub fn get_rock(&self)->&i32{
        &self.rock
    }

    //this feels bad
    pub fn has_more_than(&self,mat:&Material,compared_val:MaterialValue)->bool{

        match mat {
            Material::Money=>{ 
                let val = match compared_val {
                    MaterialValue::I32(val)=>val,
                    MaterialValue::F32(val)=>val as i32,
                };
                return self.money>=val;
            }
            Material::Rock=>{
                let val = match compared_val {
                    MaterialValue::I32(val)=>val,
                    MaterialValue::F32(val)=>val as i32,
                };
                return self.rock>=val;
            }
            
        }
    }


    pub fn subtract_from_resource(&mut self,val:MaterialValue,mat:Material){
        self.add_to_resource(-val, mat)
    }

    //reformat
    pub fn add_to_resource(&mut self,val:MaterialValue,mat:Material){
        match mat {
            Material::Money=>{
                let added_val:i32=match val {
                    MaterialValue::I32(val)=>val,
                    _=>panic!("tried to add not i32 to money"),

                };
                self.money+=added_val;
                //println!("{}",added_val);
            },
            Material::Rock=>{
                let added_val:i32=match val {
                    MaterialValue::I32(val)=>val,
                    _=>panic!("tried to add not i32 to rock"),

                };
                self.rock+=added_val;

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
            //these 2 are usef for calculating income
            //make another one that is scaleable
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

    //change to add whatever is given 
    pub fn add_resource(&mut self, building_info: &Building,multiplier:f32) {
        match &building_info.building_type {
            BuildingType::Production(mat)=>  match  mat{
                Material::Money => self.add_money((building_info.produced_amount * multiplier) as i32),
                Material::Rock => self.perm_resources.add_to_resource(MaterialValue::I32((building_info.produced_amount*multiplier)as i32), mat.clone()),
            },

            BuildingType::Buff(_)=>(),
        }
    }

    pub fn get_permanent_resources(&self)->&PermanentGameResources{
        &self.perm_resources
    }

    pub fn get_mut_perm_resources(&mut self)->&mut PermanentGameResources{
        &mut self.perm_resources
    }

    //add money to temp money
    pub fn add_money(&mut self, value: i32) {
        self.temp_money += value;
    }

    //permanent money subtract
    /*
    pub fn subtract_money(&mut self, value: i32) {
        self.perm_resources.add_to_resource(MaterialValue::I32(-value), Material::Money) ;
    }
     * */


    pub fn get_money(&self) -> &i32 {
        &self.perm_resources.get_money()
    }

    pub fn to_string(&self) -> String {
        format!(
            "money: {} \ncurrent income: {}\nrock: {}",
            self.perm_resources.get_money(), self.last_collection_income,self.perm_resources.get_rock()
        )
    }
}
