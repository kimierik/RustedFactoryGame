pub mod material;
pub mod state;

use material::Material;
use material::BuildingType;
use state::State;

use super::cordinate::Cordinates;

//make getters
pub struct Building {
    pub cost: i32,
    pub building_type:BuildingType,
    pub produced_amount: f32,
    pub cost_increase: f32,
}

impl Building {
    //make this a little better looking
    fn make_building(machine: &State) -> Self {
        match machine {
            State::FactoryBlock => Building {
                cost: 1,
                building_type:BuildingType::Production(Material::Money),
                produced_amount: 1.0,
                cost_increase: 1.0,
            },

            State::DefaultBlock => Building {
                cost: 10,
                //2.0 in building type constructor is supposed to be produced_amount
                building_type:BuildingType::Buff(vec![(2.0,Cordinates::from(0.0, -1.0))]),
                produced_amount: 2.0,
                cost_increase: 1.0,
            },
        }
    }

    pub fn to_string(&self, is: &State) -> String {
        match &self.building_type {
            BuildingType::Production(mat)=> format!(
                "          {} \nCost: {} \nAdds: {} to {}",
                is, self.cost, self.produced_amount, mat
            ),

            BuildingType::Buff(_)=> format!(
                "          {} \nCost: {} \nAdds: {} to {}",
                is, self.cost, self.produced_amount, "buff"
            ),
        }
    }



}

//different buildings and their stats
