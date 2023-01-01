use std::{time::Instant, vec, collections::HashMap};
use ggez::graphics;

pub mod cordinate;
pub mod tile;
pub mod game_resources;
pub mod player;
pub mod tile_state;

use cordinate::Cordinates;
use game_resources::GameResources;
use player::Player;
use tile::Tile;

use crate::inputs::player_actions::PlayerActions;
use crate::inputs;


//houses tilesize and camera pan offset, zoom etc
pub struct ScreenInfo {
    pub tile_size: f32,
}

pub struct MainState {
    map: Vec<Tile>,
    player: Player,
    screendata: ScreenInfo,
    resources: GameResources,
    //keeps time between resource collections
    time_since_last_collection_cycle: Instant,
    //keyboard related data, keymap and input handling related data
    input_data:inputs::keyboard_input_data::InputData,
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        MainState {
            map: vec![],
            player: Player::new(),
            screendata: ScreenInfo { tile_size: 20.0 },
            resources: GameResources::make_instance(),
            time_since_last_collection_cycle: Instant::now(),

            input_data:inputs::keyboard_input_data::InputData::new(),
        }
    }

    pub fn change_player_location_x(&mut self, x: f32) {
        self.player.add_cords(&Cordinates::from(x, 0.0));
    }

    pub fn change_player_location_y(&mut self, y: f32) {
        self.player.add_cords(&Cordinates::from(0.0, y));
    }


    pub fn check_and_place_tile(&mut self, state: tile_state::State) {
        if self.resources.get_money() >= &state.get_cost_for_tile(){
            if !self.player_is_on_tile() {
                self.resources.subtract_money(state.get_cost_for_tile());
                self.map.push(Tile::create_tile_with(
                    self.get_player_ref().get_cords().clone(),
                    state,
                ))
            }
        }
    }

    pub fn check_and_remove_tile(&mut self){
        if self.player_is_on_tile(){
            //should always return a tile
            let data=self.get_tile_on_player().unwrap();
            let tile_cost= data.0.get_state().get_cost_for_tile();
            self.map.remove(data.1);
            self.resources.add_money(tile_cost);
        }
    }

    pub fn get_player_ref(&self) -> &Player {
        &self.player
    }

    fn player_is_on_tile(&self) -> bool {
        for tile in self.map.iter() {
            if tile.is_here(self.player.get_cords()) {
                return true;
            }
        }
        false
    }

    //gets the tile that the player is standing on
    fn get_tile_on_player(&self)->Option<(&Tile,usize)>{
        for (index,tile) in self.map.iter().enumerate() {
            if tile.is_here(self.player.get_cords()) {
                return Some((tile,index));
            }
        }
        None
    }

    //loop and apply things
    pub fn loop_tiles_and_apply_effect(&mut self) {
        for item in self.map.iter() {
            item.apply_effect(&mut self.resources);
        }
        self.resources.collapse_money();
    }

    pub fn resources_as_string(&self) -> String {
        self.resources.to_string()
    }

    pub fn get_recource_drawable(&self) -> graphics::Text {
        let data = self.resources_as_string();
        let mut txt = graphics::Text::new(data);
        txt.set_bounds([crate::UIX, crate::UIY + crate::GAME_SCREENY]);
        txt
    }

    pub fn get_time_since_collect(&self) -> &Instant {
        &self.time_since_last_collection_cycle
    }

    pub fn get_screen_info(&self) -> &ScreenInfo {
        &self.screendata
    }

    pub fn get_map(&self)->&Vec<Tile>{
        &self.map
    }

    pub fn get_key_map(&self)->&HashMap<ggez::input::keyboard::KeyCode,PlayerActions>{
        &self.input_data.key_map
    }
    pub fn get_input_data(&self)->&inputs::keyboard_input_data::InputData{
        &self.input_data
    }

    pub fn get_mut_input_data(&mut self)->&mut inputs::keyboard_input_data::InputData{
        &mut self.input_data
    }

    pub fn reset_time_since_collect(&mut self) {
        self.time_since_last_collection_cycle = Instant::now();
    }

}
