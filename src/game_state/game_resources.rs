use json::JsonValue;
use strum::IntoEnumIterator;

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

    pub fn get_current_materials(&self)->Vec<(Material,MaterialValue)>{
        let mut ret :Vec<(Material,MaterialValue)>=vec![];
        for mat in Material::iter(){
            ret.push((mat,self.get_material(mat)));
        }
        ret
    }


    pub fn get_serialisable_materials_info()->Vec<(Material,MaterialValue)>{
        let mut retvec=vec![];
        for material in Material::iter(){
            retvec.push((material,Self::get_default_material(material)))
        }
        retvec
    }


    //TODO this needs to be changed to work with f32 
    //currently only calling getint
    //maybe implement MaterialValue into JsonValue
    pub fn get_as_serialisable(&self)->JsonValue{
        let mut returned_json = json::JsonValue::new_object();
        for material in Material::iter(){
            returned_json[material.to_string()]=self.get_material(material).get_int().into();

        }
        returned_json
    }



    pub fn get_material(&self,mat:Material)->MaterialValue{
        match mat {
            Material::Rock=>MaterialValue::I32(self.rock),
            Material::Money=>MaterialValue::I32(self.money),
        }
    }

    pub fn get_default_material(mat:Material)->MaterialValue{
        match mat {
            Material::Rock=>MaterialValue::I32(0),
            Material::Money=>MaterialValue::I32(0),
        }
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
    material_last_cycle_income:Vec<(Material,MaterialValue)>,
    temp_money: i32,
    last_collection_income: i32,
}

impl GameResources {
    pub fn make_instance() -> Self {
        GameResources {
            perm_resources:PermanentGameResources::create_empty(),

            material_last_cycle_income:vec![],
            //these 2 are usef for calculating income
            //make another one that is scaleable
            temp_money: 0,
            last_collection_income: 0,
        }
    }

    //this needs to change if we want to make it scalable, cannot change both of these every time
    //mayby make saved res its own thing

    pub fn make_instance_with_permanent(res:PermanentGameResources)->Self{
        GameResources { perm_resources: res, temp_money: 0,  last_collection_income: 0 ,
            material_last_cycle_income:vec![],}
    }

    pub fn set_income_db(&mut self, new_db:Vec<(Material,MaterialValue)>){
        self.material_last_cycle_income=new_db;
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

    pub fn get_income_string(&self)->String{
        let mut ret :String=String::new();
        for (mat,matval) in self.material_last_cycle_income.iter(){
            ret+= format!("{} produced : {} \n",mat,matval.get_string()).as_str();
        }
        ret
    }

    pub fn to_string(&self) -> String {
        format!(
            "money: {} \nrock: {} \n Income: \n{}",
            self.perm_resources.get_material(Material::Money), self.perm_resources.get_material(Material::Rock).get_int(), self.get_income_string()
        )
    }
}
