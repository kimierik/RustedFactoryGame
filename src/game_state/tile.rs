use super::Cordinates;
use super::GameResources;
use super::ScreenInfo;
use super::tile_state::State;
use ggez::graphics;

//move state away to another file


pub struct Tile {
    cords: Cordinates,
    state: State,
}

impl Tile {
    pub fn create_tile_with(tile_location: Cordinates, machine: State) -> Self {
        Tile {
            cords: tile_location,
            state: machine,
        }
    }


    //todo implement world to screen cordinates
    pub fn get_drawable(
        &self,
        ctx: &mut ggez::Context,
        screen: &ScreenInfo,
    ) -> Result<graphics::Mesh, ggez::GameError> {
        let world_cords = self.cords.world_to_screen(screen);
        let color: ggez::graphics::Color = self.state.get_color();
        let p = ggez::graphics::Rect::new(
            world_cords.x,
            world_cords.y,
            screen.get_tile_size(),
            screen.get_tile_size(),
        );
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), p, color);
        mesh
    }

    pub fn is_here(&self, cords: &Cordinates) -> bool {
        self.cords.is_same_as(cords)
    }

    pub fn apply_effect(&self, resouce_pool: &mut GameResources) {
       resouce_pool.add_resource(self.get_state().get_building_info()) 
    }

    pub fn get_state(&self)->&State{
        &self.state
    }
}
