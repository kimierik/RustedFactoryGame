use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Material {
    Money,
    MoneyMultiplier,
}

//maybe too much to change per material 
//dead code in here bc functions are usefull in future

impl Material{
    #[allow(dead_code)]
    pub fn convert_to_permanent_material(&self)->PermanentMaterial{
        match self {
            Material::Money=>PermanentMaterial::Money,
            _=>panic!("not supported conversion {}",self),
        }
    }
}


impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}




#[derive(EnumIter, Debug, Clone, Copy)]
pub enum PermanentMaterial{
    Money,
}

impl PermanentMaterial{
    #[allow(dead_code)]
    pub fn convert_to_normal_material(&self)->Material{
        match self {
            PermanentMaterial::Money=>Material::Money,
        }
    }

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



impl std::fmt::Display for PermanentMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

