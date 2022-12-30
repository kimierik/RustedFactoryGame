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
    //keyboard map
    key_map:HashMap<ggez::input::keyboard::KeyCode,PlayerActions>,
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        MainState {
            map: vec![],
            player: Player::new(),
            screendata: ScreenInfo { tile_size: 20.0 },
            resources: GameResources::make_instance(),
            time_since_last_collection_cycle: Instant::now(),
            key_map:inputs::update_key_bindings(),
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

    pub fn get_player_ref(&self) -> &Player {
        &self.player
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
        &self.key_map
    }

    pub fn reset_time_since_collect(&mut self) {
        self.time_since_last_collection_cycle = Instant::now();
    }

    fn player_is_on_tile(&self) -> bool {
        for tile in self.map.iter() {
            if tile.is_here(self.player.get_cords()) {
                return true;
            }
        }
        false
    }
}
