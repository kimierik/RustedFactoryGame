use super::game_state::buildings::state::State;
use super::MainState;
use ggez;
use ggez::input::keyboard::KeyCode;

pub mod keybind_setup;
pub mod keyboard_input_data;
pub mod player_actions;
use player_actions::PlayerActions;

//todo make this not shit
//
//
//public interfaces
//handle KeyboardInputActions
//there are orphan functions in this file. do something about them

//finds key from the hash keybindings and activates its effect if there is one
fn activate_key(key: &KeyCode, game: &mut MainState) {
    let action = game.get_key_map().get(key);
    match action {
        Some(action) => action.apply_effect(game),
        None => (), //no key found in keymap
    }
}

//loops all keys and activates its effect if it exists
pub fn handle_keyboard_inputs(game: &mut MainState, ctx: &mut ggez::Context) {
    let currently_pressed_keys = ctx.keyboard.pressed_keys();
    //loop through all currently pressed keys and see if we have allready handled them
    for key in currently_pressed_keys {
        if !game.get_input_data().is_key_handled(key) {
            activate_key(key, game);
            game.get_mut_input_data().handled_keys.push(key.clone());
        }
    }

    //remove handled key if it is not in currently_pressed_keys
    //this way we can have a debounce
    let mut keys_to_remove_from_handled: Vec<KeyCode> = vec![];
    for handled_key in game.get_input_data().handled_keys.iter() {
        let mut is_in_pressed = false;

        for key in currently_pressed_keys {
            if handled_key == key {
                is_in_pressed = true;
            }
        }
        if !is_in_pressed {
            keys_to_remove_from_handled.push(handled_key.clone())
        }
    }

    //seacond loop so that the first mut ref of game goes out of scope
    //remove keys from handled_keys
    for i in keys_to_remove_from_handled.iter() {
        game.get_mut_input_data()
            .handled_keys
            .retain(|value| *value != i.clone());
    }
}
