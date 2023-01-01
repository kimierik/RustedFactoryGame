
use strum_macros::EnumIter;


pub enum Material{
    Money,
    MoneyMultiplier,
}



//make getters
pub struct Building{
    pub cost:i32,
    pub created_material:Material,
    pub produced_amount:f32,
    pub cost_increase:f32,
}

impl Building{

    //make this a little better looking
    fn make_building(machine:&State)->Self{
        match machine {
            State::FactoryBlock=>Building { cost: 1, created_material: Material::Money, produced_amount: 1.0, cost_increase: 1.0 },
            State::DefaultBlock=>Building { cost: 10, created_material: Material::MoneyMultiplier, produced_amount: 0.01, cost_increase: 1.0 },
        }
    }
}




//different buildings and their stats 
#[derive(EnumIter, Debug,Clone,Copy)]
pub enum State {
    FactoryBlock,
    DefaultBlock,
}

impl State{

    pub fn get_building_info(&self)->Building{
        Building::make_building(self)
    }

    pub fn get_color(&self) -> ggez::graphics::Color {
        match self {
            State::DefaultBlock => ggez::graphics::Color::BLUE,
            State::FactoryBlock => ggez::graphics::Color::GREEN,
        }
    }

    pub fn get_cost_for_tile(&self)->i32{
        self.get_building_info().cost
    }

}
