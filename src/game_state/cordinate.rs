use super::ScreenInfo;

//global cord struct
#[derive(Clone)]
pub struct Cordinates {
    pub x: f32,
    pub y: f32,
}

impl Cordinates {
    pub fn from(xn: f32, yn: f32) -> Self {
        Cordinates { x: xn, y: yn }
    }

    //figureout how to do this properly
    pub fn world_to_screen(&self, screen: &ScreenInfo) -> Cordinates {
        Cordinates::from(self.x * screen.tile_size, self.y * screen.tile_size)
    }

    //compare 2 Cordinates and see if they are the same
    pub fn is_same_as(&self, cord: &Cordinates) -> bool {
        return self.x == cord.x && self.y == cord.y;
    }
}
