pub mod material;
pub mod state;

use material::Material;
use state::State;
use ggez::graphics::Color;

use super::cordinate::Cordinates;


#[derive( Debug, Clone)]
pub enum BuildingType{
    Production(Material),

    //f32 is the amount that cordinate is buffed by
    Buff(Vec<(f32,Cordinates)>),//affected tiles relative to the tiles own cordinates
}


//make getters
//needs what material this building costs
pub struct Building {
    pub cost_material:Material,
    pub cost: i32,
    pub building_type:BuildingType,
    pub produced_amount: f32,
    pub cost_increase: f32,
    pub color:Color,
}

impl Building {
    //make this a little better looking
    fn make_building(machine: &State) -> Self {
        match machine {
            State::FactoryBlock => Building {
                cost_material:Material::Money,
                cost: 1,
                building_type:BuildingType::Production(Material::Money),
                produced_amount: 1.0,
                cost_increase: 1.0,
                color:Color::GREEN,
            },

            State::DefaultBlock => Building {
                cost_material:Material::Money,
                cost: 10,
                //2.0 in building type constructor is supposed to be produced_amount
                building_type:BuildingType::Buff(vec![(2.0,Cordinates::from(0.0, -1.0))]),
                produced_amount: 2.0,
                cost_increase: 1.0,
                color:Color::BLUE,
            },
            State::RockMine => Building {
                cost_material:Material::Money,
                cost: 100,
                building_type:BuildingType::Production(Material::Rock),
                produced_amount: 1.0,
                cost_increase: 10.0,
                color:Color::RED,
            },
        }
    }


    pub fn to_string(&self, is: &State) -> String {
        match &self.building_type {
            BuildingType::Production(mat)=> format!(
                "          {} \nCost: {} {} \nAdds: {} to {}",
                is, self.cost,self.cost_material ,self.produced_amount, mat
            ),

            BuildingType::Buff(_)=> format!(
                "          {} \nCost: {} {} \nAdds: {} to {}",
                is, self.cost, self.cost_material,self.produced_amount, "buff"
            ),
        }
    }



}

//different buildings and their stats
