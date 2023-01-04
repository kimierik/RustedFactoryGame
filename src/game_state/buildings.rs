pub mod material;
pub mod state;

use material::Material;
use state::State;

//make getters
pub struct Building {
    pub cost: i32,
    pub created_material: Material,
    pub produced_amount: f32,
    pub cost_increase: f32,
}

impl Building {
    //make this a little better looking
    fn make_building(machine: &State) -> Self {
        match machine {
            State::FactoryBlock => Building {
                cost: 1,
                created_material: Material::Money,
                produced_amount: 1.0,
                cost_increase: 1.0,
            },

            State::DefaultBlock => Building {
                cost: 10,
                created_material: Material::MoneyMultiplier,
                produced_amount: 0.01,
                cost_increase: 1.0,
            },
        }
    }

    pub fn to_string(&self, is: &State) -> String {
        format!(
            "          {} \nCost: {} \nAdds: {} to {}",
            is, self.cost, self.produced_amount, self.created_material
        )
    }
}

//different buildings and their stats
