use super::{Building, BuildingType};
use ggez::graphics;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum State {
    FactoryBlock,
    DefaultBlock,
    RockMine,
}

impl State {
    pub fn get_building_info(&self) -> Building {
        Building::make_building(self)
    }

    pub fn get_building_type(&self)->BuildingType{
        Building::make_building(self).building_type
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
            State::RockMine => ggez::graphics::Color::RED,
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
