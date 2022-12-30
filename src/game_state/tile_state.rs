
//different buildings and their stats 
pub enum State {
    FactoryBlock,
    DefaultBlock,
}

impl State{

    pub fn get_color(&self) -> ggez::graphics::Color {
        match self {
            State::DefaultBlock => ggez::graphics::Color::BLUE,
            State::FactoryBlock => ggez::graphics::Color::GREEN,
        }
    }

    pub fn get_cost_for_tile(&self)->i32{
        match self {
            State::DefaultBlock => 10,
            State::FactoryBlock => 1,
        }
    }

}
