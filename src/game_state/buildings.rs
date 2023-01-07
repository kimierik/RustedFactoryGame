pub mod material;
pub mod state;

use ggez::graphics::Color;
use material::Material;
use state::State;

use super::cordinate::Cordinates;






#[derive( Debug, Clone)]
pub enum BuildingType{
    Production(Material),
    Buff(Vec<(f32,Cordinates)>),//affected tiles relative to the tiles own cordinates
}


//make getters
#[derive( Debug, Clone)]
pub struct Building {
    pub cost: i32,
    pub building_type:BuildingType,
    pub produced_amount: f32,
    pub cost_increase: f32,
    pub color:ggez::graphics::Color,
}

impl Building {
    //make this a little better looking

    pub fn make_building(machine: &State) -> Self {
        match machine {
            State::FactoryBlock(_) => Building {
                cost: 1,
                building_type:BuildingType::Production(Material::Money),
                produced_amount: 1.0,
                cost_increase: 1.0,
                color:Color::GREEN, 
            },

            State::DefaultBlock(_) => Building {
                cost: 10,
                //2.0 in building type constructor is supposed to be produced_amount
                building_type:BuildingType::Buff(vec![(2.0,Cordinates::from(0.0, -1.0))]),
                produced_amount: 2.0,
                cost_increase: 1.0,
                color:Color::BLUE, 
            },

            State::RockMine(_) => Building {
                cost: 100,
                building_type:BuildingType::Production(Material::Rock),
                produced_amount: 1.0,
                cost_increase: 10.0,
                color:Color::RED, 
            },
        }
    }



    //give some identifier and returns a building. this is used in constructiog the enums
    //wrappaer for State::FactoryBlock(Building::make_building(State::FactoryBlock(Building::default())))
    pub fn create_building(machine: &State)->State{
        match machine {
            State::FactoryBlock(_) => State::FactoryBlock( Building {
                cost: 1,
                building_type:BuildingType::Production(Material::Money),
                produced_amount: 1.0,
                cost_increase: 1.0,
                color:Color::GREEN, 
            }),

            State::DefaultBlock(_) => State::DefaultBlock( Building {
                cost: 10,
                //2.0 in building type constructor is supposed to be produced_amount
                building_type:BuildingType::Buff(vec![(2.0,Cordinates::from(0.0, -1.0))]),
                produced_amount: 2.0,
                cost_increase: 1.0,
                color:Color::BLUE, 
            }),

            State::RockMine(_) => State::RockMine( Building {
                cost: 100,
                building_type:BuildingType::Production(Material::Rock),
                produced_amount: 1.0,
                cost_increase: 10.0,
                color:Color::RED, 
            }),
        }

        
    }



    /*
     * makes state.to_string readable
     * */
    fn clean_state_string(inp:String)->String{
        let mut output:String=String::new();
        for character in inp.into_bytes().into_iter(){
            if character as char=='('{
                break;
            }
            output+= &((character as char).to_string());
        }
        output
    }


    pub fn to_string(&self, is: &State) -> String {

        let asd = Self::clean_state_string(is.to_string());

        match &self.building_type {
            BuildingType::Production(mat)=> format!(
                "          {} \nCost: {} \nAdds: {} to {}",
                asd, self.cost, self.produced_amount, mat
            ),

            BuildingType::Buff(_)=> format!(
                "          {} \nCost: {} \nAdds: {} to {}",
                asd, self.cost, self.produced_amount, "buff"
            ),
        }
    }



}



//this is implemented so we can itereate over state to get the graphics
impl Default for Building{
    fn default() -> Self {
        Building{
            cost: 1,
            building_type:BuildingType::Production(Material::Money),
            produced_amount: 1.0,
            cost_increase: 1.0,
            color:Color::GREEN, 
        }
    }

}

