use strum_macros::EnumIter;
use strum::IntoEnumIterator;


//simple way to give i32 or f32 as argument witout using generics
#[allow(dead_code)]
#[derive(Debug,Clone,Copy)]
pub enum MaterialValue{
    I32(i32),
    F32(f32),
}


#[derive(EnumIter, Debug, Clone,Copy)]
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



impl MaterialValue{
    pub fn get_int(&self)->i32{
        match self {
            Self::I32(bal)=>bal.clone(),
            Self::F32(bal)=>bal.clone() as i32,
        }
    }
    pub fn get_float(&self)->f32{
        match self {
            Self::I32(bal)=>bal.clone() as f32,
            Self::F32(bal)=>bal.clone() ,
        }
    }
    pub fn get_string(&self)->String{
        match self {
            Self::I32(val)=>val.to_string(),
            Self::F32(val)=>val.to_string(),
        }

    }

}



//this is implemented so we can flip a material value
//usefull for minus operations
impl std::ops::Neg for MaterialValue{
    type Output = MaterialValue;
    fn neg(self) -> Self::Output {
        match self {
            MaterialValue::I32(val)=>MaterialValue::I32(-val),
            MaterialValue::F32(val)=>MaterialValue::F32(-val),
        }
    }

} 


//if mvi32-mvi32 should return mvi32
impl std::ops::Sub for MaterialValue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::I32(val)=>{
                Self::I32(val-rhs.get_int())
            },

            Self::F32(val)=>{
                Self::F32(val-rhs.get_float())
            }
        }
    }
}

impl std::ops::Add for MaterialValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        -(-self-rhs)
    }
    
}





impl std::fmt::Display for MaterialValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(val)=> write!(f, "{:?}", val),
            Self::F32(val)=>write!(f, "{:?}", val),
        }
    }
}

