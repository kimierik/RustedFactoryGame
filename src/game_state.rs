use ggez::graphics;
use std::{collections::HashMap, time::Instant, vec};

pub mod buildings;
pub mod cordinate;
pub mod game_resources;
pub mod player;
pub mod screen_info;
pub mod tile;

use buildings::state::State;
use cordinate::Cordinates;
use game_resources::GameResources;
use player::Player;
use screen_info::ScreenInfo;
use tile::Tile;

use ggez_egui::EguiBackend;

use crate::inputs;
use crate::inputs::player_actions::PlayerActions;

use self::buildings::BuildingType;
use self::game_resources::PermanentGameResources;

pub struct MainState {
    map: Vec<Tile>,
    player: Player,
    screendata: screen_info::ScreenInfo,
    resources: GameResources,
    //keeps time between resource collections
    time_since_last_collection_cycle: Instant,
    //keyboard related data, keymap and input handling related data
    input_data: inputs::keyboard_input_data::InputData,

    pub egui_backend:EguiBackend,
}

impl MainState {
    pub fn new() -> Self {
        MainState {
            map: vec![],
            player: Player::new(),
            screendata: screen_info::ScreenInfo::new(),
            resources: GameResources::make_instance(),
            time_since_last_collection_cycle: Instant::now(),

            input_data: inputs::keyboard_input_data::InputData::new(),
            egui_backend:EguiBackend::default(),

        }
    }

    pub fn new_from_save(mapvec: Vec<Tile>, materials: PermanentGameResources) -> Self {
        MainState {
            map: mapvec,
            player: Player::new(),
            screendata: screen_info::ScreenInfo::new(),
            resources: GameResources::make_instance_with_permanent(materials),
            time_since_last_collection_cycle: Instant::now(),
            input_data: inputs::keyboard_input_data::InputData::new(),
            egui_backend:EguiBackend::default(),
        }
    }

    pub fn hotload_data(&mut self,newgame:Self){
        self.map=newgame.map;
        self.resources=newgame.resources;
    }


    pub fn change_player_location_x(&mut self, x: f32) {
        self.player.add_cords(&Cordinates::from(x, 0.0));
    }

    pub fn change_player_location_y(&mut self, y: f32) {
        self.player.add_cords(&Cordinates::from(0.0, y));
    }

    //make this to be able to check what the buildings chosts
    pub fn check_and_place_tile(&mut self, state: State) {
        let building=state.get_building_info();
        //if we have enough material for the material that the building needs
        if self.resources.get_permanent_resources().has_more_than(&building.cost_material,buildings::material::MaterialValue::I32(building.cost)){
            if !self.player_is_on_tile() {
                self.resources.get_mut_perm_resources().subtract_from_resource(buildings::material::MaterialValue::I32(state.get_cost_for_tile()),building.cost_material);
                self.map.push(Tile::create_tile_with(
                    self.get_player_ref().get_cords().clone(),
                    state,
                ));
                //is 2x even when this is off
                self.loop_tiles_and_apply_buffs();
            }
        }
    }

    pub fn check_and_remove_tile(&mut self) {
        if self.player_is_on_tile() {
            //should always return a tile
            let data = self.get_tile_on_player().unwrap();
            let tile_cost = data.0.get_state().get_cost_for_tile();
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

    fn get_mut_tile_from_cord(&mut self ,cord:Cordinates)->Option<&mut Tile>{
        for tile in &mut self.map {
            if tile.is_here(&cord) {
                return Some(tile);
            }
        }
        None
    }

    //gets the tile that the player is standing on
    fn get_tile_on_player(&self) -> Option<(&Tile, usize)> {
        for (index, tile) in self.map.iter().enumerate() {
            if tile.is_here(self.player.get_cords()) {
                return Some((tile, index));
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

    
    pub fn loop_tiles_and_apply_buffs(&mut self){

        let mut tiles_to_buff:Vec<Vec<(f32,Cordinates)>> =vec![];

        //hmm
        for tile in self.map.iter(){
            //we should not match here, call buff match the option to see if it is buff  builfin
            match tile.get_state().get_building_type() {
                BuildingType::Buff(cords)=>tiles_to_buff.push(tile.get_buffed_cords(cords)),
                _=>(),//only buff buildings
            }
        }

        // cannot happen in same loop because rust
        for buffed_area in tiles_to_buff{
            for (value,cord) in buffed_area{
                match self.get_mut_tile_from_cord(cord) {
                    Some(tile)=>{tile.reset_buffs(); tile.add_buff(value);},
                    None=>(),
                };
            }
        }


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

    //gettes so we can have private fields
    pub fn get_time_since_collect(&self) -> &Instant {
        &self.time_since_last_collection_cycle
    }

    pub fn get_screen_info(&self) -> &ScreenInfo {
        &self.screendata
    }

    pub fn get_mut_screen_info(&mut self) -> &mut ScreenInfo {
        &mut self.screendata
    }

    pub fn get_map(&self) -> &Vec<Tile> {
        &self.map
    }

    pub fn get_key_map(&self) -> &HashMap<String, PlayerActions> {
        &self.input_data.key_map
    }
    pub fn get_input_data(&self) -> &inputs::keyboard_input_data::InputData {
        &self.input_data
    }

    pub fn get_mut_input_data(&mut self) -> &mut inputs::keyboard_input_data::InputData {
        &mut self.input_data
    }

    pub fn reset_time_since_collect(&mut self) {
        self.time_since_last_collection_cycle = Instant::now();
    }

    pub fn get_resource(&self) -> &GameResources {
        &self.resources
    }

    pub fn get_mut_resource(&mut self) -> &mut GameResources {
        &mut self.resources
    }
}
