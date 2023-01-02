use ggez::graphics;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;



//BREAK this file



#[derive(EnumIter, Debug, Clone, Copy)]
pub enum Material {
    Money,
    MoneyMultiplier,
}
impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

    pub fn to_string(&self,is:&State) -> String {
        format!(
            "          {} \nCost: {} \nAdds: {} to {}",
            is,self.cost, self.produced_amount, self.created_material
        )
    }
}

//different buildings and their stats
#[derive(EnumIter, Debug, Clone, Copy)]
pub enum State {
    FactoryBlock,
    DefaultBlock,
}

impl State {
    pub fn get_building_info(&self) -> Building {
        Building::make_building(self)
    }


    pub fn get_enum_from_string(find: &str) -> Self {
        for i in State::iter() {
            if i.to_string() == find {
                return i;
            }
        }
        panic!("load data parse error. enum name incorrect")
    }

    pub fn get_color(&self) -> ggez::graphics::Color {
        match self {
            State::DefaultBlock => ggez::graphics::Color::BLUE,
            State::FactoryBlock => ggez::graphics::Color::GREEN,
        }
    }

    pub fn get_cost_for_tile(&self) -> i32 {
        self.get_building_info().cost
    }

    pub fn get_building_drawable(&self) -> graphics::Text {
        let data = self.get_building_info().to_string(self);
        let mut txt = graphics::Text::new(data);
        txt.set_bounds([crate::UIX, crate::UIY + crate::GAME_SCREENY]);
        txt
    }
}



impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
