use crate::{game_state::cordinate::Cordinates, serialisation};

use super::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum PlayerActions {
    MovePlayerUp,
    MovePlayerDown,
    MovePlayerLeft,
    MovePlayerRight,

    MoveCameraUp,
    MoveCameraDown,
    MoveCameraLeft,
    MoveCameraRight,

    Demolish,
    MakeFactory,
    MakeDefault,
    NoAction,

    SaveGame,
    DebugKey,

}

impl std::fmt::Display for PlayerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PlayerActions {
    pub fn apply_effect(self, game: &mut super::MainState) {
        match self {
            PlayerActions::MovePlayerUp => game.change_player_location_y(-1.0),
            PlayerActions::MovePlayerDown => game.change_player_location_y(1.0),
            PlayerActions::MovePlayerLeft => game.change_player_location_x(-1.0),
            PlayerActions::MovePlayerRight => game.change_player_location_x(1.0),

            PlayerActions::MoveCameraUp => game
                .get_mut_screen_info()
                .offset_pan(Cordinates { x: 0.0, y: -1.0 }),
            PlayerActions::MoveCameraDown => game
                .get_mut_screen_info()
                .offset_pan(Cordinates { x: 0.0, y: 1.0 }),
            PlayerActions::MoveCameraLeft => game
                .get_mut_screen_info()
                .offset_pan(Cordinates { x: -1.0, y: 0.0 }),
            PlayerActions::MoveCameraRight => game
                .get_mut_screen_info()
                .offset_pan(Cordinates { x: 1.0, y: 0.0 }),

            PlayerActions::Demolish => game.check_and_remove_tile(),
            PlayerActions::MakeFactory => game.check_and_place_tile(State::FactoryBlock),
            PlayerActions::MakeDefault => game.check_and_place_tile(State::DefaultBlock),
            PlayerActions::NoAction => (),

            PlayerActions::SaveGame=>serialisation::save_game(game),
            PlayerActions::DebugKey=>println!("debug key pressed"),

        }
    }

    pub fn get_enum_from_string(find: &str) -> Self {
        for i in PlayerActions::iter() {
            if i.to_string() == find {
                return i;
            }
        }
        return PlayerActions::NoAction;
    }
}
