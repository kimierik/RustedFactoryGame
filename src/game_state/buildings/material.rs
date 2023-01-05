use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::game_state::cordinate::Cordinates;


#[derive( Debug, Clone)]
pub enum BuildingType{
    Production(Material),
    Buff(Vec<(f32,Cordinates)>),//affected tiles relative to the tiles own cordinates
}



#[derive(EnumIter, Debug, Clone)]
pub enum Material {
    Money,
}

//maybe too much to change per material 
//dead code in here bc functions are usefull in future

impl Material{

    #[allow(dead_code)]
    pub fn get_material_from_string(find:String)->Self{
        for i in Self::iter() {
            if i.to_string() == find {
                return i;
            }
        }
        panic!("errro no material named {}",find);// TODO change panic to option
    }
}


impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


