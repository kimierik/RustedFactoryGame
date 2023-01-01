use super::Cordinates;
//houses tilesize and camera pan offset, zoom etc
//move to own file
pub struct ScreenInfo {
    tile_size: f32,
    pan_offset: Cordinates,
}

impl ScreenInfo {
    pub fn new() -> Self {
        ScreenInfo {
            tile_size: 20.0,
            pan_offset: Cordinates::from(0.0, 0.0),
        }
    }
    pub fn get_tile_size(&self) -> f32 {
        self.tile_size
    }

    pub fn get_pan(&self) -> &Cordinates {
        &self.pan_offset
    }

    pub fn offset_pan(&mut self, cords: Cordinates) {
        self.pan_offset = self.pan_offset + cords;
    }

    pub fn set_pan(&mut self, cords: Cordinates) {
        self.pan_offset = cords;
    }
}
