use super::buildings::material::BuildingType;
use super::buildings::state::State;
use super::Cordinates;
use super::GameResources;
use super::ScreenInfo;
use ggez::graphics;
use json::array;

//move state away to another file

#[derive(Debug)]
pub struct Tile {
    cords: Cordinates,
    state: State,
    pub productivity_multiplier:f32,
}

impl Tile {
    pub fn create_tile_with(tile_location: Cordinates, machine: State) -> Self {
        Tile {
            cords: tile_location,
            state: machine,
            productivity_multiplier:1.0,
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
        match &self.get_state().get_building_type() {
            BuildingType::Production(_)=> resouce_pool.add_resource(&self.get_state().get_building_info(),self.productivity_multiplier),
            BuildingType::Buff(_)=>(),
            
        }
    }

    //offsets given cordinate vector with its own cordinates
    fn get_ofsetted_cordinates(&self,cords:&Vec< (f32,Cordinates)>)->Vec<(f32,Cordinates)>{
        let mut retvec:Vec<(f32,Cordinates)> =vec![];
        for (va,cord) in cords{
            retvec.push((va.clone(),self.cords+cord.clone()));
        }
        retvec
    }

    pub fn add_buff(&mut self,val:f32){
        self.productivity_multiplier+=val
    }

    pub fn reset_buffs(&mut self){
        self.productivity_multiplier=1.0;
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }


    //returns world cordinates this tile is attempting to buff
    pub fn get_buffed_cords(&self,cords:Vec<(f32,Cordinates)>)-> Vec<(f32,Cordinates)>{
         self.get_ofsetted_cordinates(&cords)
    }

    pub fn get_as_serialisable(&self) -> json::JsonValue {
        array![self.cords.x, self.cords.y, self.state.to_string()]
    }


}
