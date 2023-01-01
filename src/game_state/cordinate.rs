use super::ScreenInfo;

//global cord struct
#[derive(Clone, Copy)]
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
        let panned_cordinates = self - screen.get_pan();
        Cordinates::from(
            panned_cordinates.x * screen.get_tile_size(),
            panned_cordinates.y * screen.get_tile_size(),
        )
    }

    //compare 2 Cordinates and see if they are the same
    pub fn is_same_as(&self, cord: &Cordinates) -> bool {
        return self.x == cord.x && self.y == cord.y;
    }
}

//QOL implementations
impl std::ops::Add<Cordinates> for Cordinates {
    type Output = Cordinates;
    fn add(self, addable: Cordinates) -> Cordinates {
        Cordinates {
            x: self.x + addable.x,
            y: self.y + addable.y,
        }
    }
}

impl std::ops::Add<&Cordinates> for &Cordinates {
    type Output = Cordinates;
    fn add(self, addable: &Cordinates) -> Cordinates {
        Cordinates {
            x: self.x + addable.x,
            y: self.y + addable.y,
        }
    }
}

impl std::ops::Sub<&Cordinates> for &Cordinates {
    type Output = Cordinates;
    fn sub(self, addable: &Cordinates) -> Cordinates {
        Cordinates {
            x: self.x - addable.x,
            y: self.y - addable.y,
        }
    }
}
