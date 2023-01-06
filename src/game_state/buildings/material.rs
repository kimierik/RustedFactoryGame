use strum_macros::EnumIter;
use strum::IntoEnumIterator;


//simple way to give i32 or f32 as argument witout using generics
#[allow(dead_code)]
pub enum MaterialValue{
    I32(i32),
    F32(f32),
}


#[derive(EnumIter, Debug, Clone)]
pub enum Material {
    Money,
    Rock,
}


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


