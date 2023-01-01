use super::Cordinates;

pub struct Player {
    cords: Cordinates,
}

impl Player {
    pub fn new() -> Self {
        Player {
            cords: Cordinates::from(0.0, 0.0),
        }
    }

    #[allow(dead_code)]
    pub fn set_cords(&mut self, new_cords: &Cordinates) {
        self.cords.x = new_cords.x;
        self.cords.y = new_cords.y;
    }

    pub fn get_cords(&self) -> &Cordinates {
        &self.cords
    }

    pub fn add_cords(&mut self, cords_to_add: &Cordinates) {
        self.cords.x += cords_to_add.x;
        self.cords.y += cords_to_add.y;
    }
}
