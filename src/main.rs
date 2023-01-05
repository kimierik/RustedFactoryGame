use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color};
use std::time::Duration;
use strum::IntoEnumIterator;

mod drawables_trait;
mod game_state;
mod inputs;
mod serialisation;
//cordinate and input does not need to be in game state
//lets do propper implementation

use drawables_trait::MakeDrawable;
use game_state::buildings::state::State;
use game_state::MainState;

use crate::serialisation::GameOptions;

const GAME_SCREENW: f32 = 600.0;
const GAME_SCREENY: f32 = 600.0;
const UIX: f32 = 300.0;
const UIY: f32 = 300.0;
const KEYBIND_FILENAME: &str = "KeyboardInputActions.cfg";

impl EventHandler<ggez::GameError> for game_state::MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        //handle keyboard inputs. and do appropriate reactions
        inputs::handle_keyboard_inputs(self, ctx);
        //do loop through all tiles and make money out of them
        //this needs to be put to a stopwatch
        if self.get_time_since_collect().elapsed() > Duration::from_secs(1) {
            self.loop_tiles_and_apply_effect();
            self.reset_time_since_collect();
        }

        //println!("{:?}",self.get_map());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        //^clears the screen with white

        //self.draw_map(&mut canvas, ctx)?;
        self.draw_object(&mut canvas, ctx, self.get_screen_info())?;

        //draw player
        self.get_player_ref()
            .draw_object(&mut canvas, ctx, self.get_screen_info())?;

        //draw the bg for the ui
        drawables_trait::draw_ui_bg(&mut canvas, ctx)?;

        //draw resources
        canvas.draw(
            &self.get_recource_drawable(),
            graphics::DrawParam::default().dest([GAME_SCREENW, 10.0]),
        );

        //loop through all states and make the thing
        //remove magic numbers
        for (index, state) in State::iter().enumerate() {
            drawables_trait::make_rect(
                game_state::cordinate::Cordinates {
                    x: index as f32 * 300.0,
                    y: GAME_SCREENY + UIY / 3.0,
                },
                300.0,
                200.0,
                state.get_color(),
                ctx,
                &mut canvas,
            )?;
            canvas.draw(
                &state.get_building_drawable(),
                graphics::DrawParam::default()
                    .dest([300.0 * index as f32, GAME_SCREENY + UIY / 2.0]),
            );
        }

        //vv puts everything we just draw to the ctx
        canvas.finish(ctx)
    }
}

//TODO
//maybe make loop into its own function
//give possibility to name the save 
//when ypu input save it asks for the save name. then it names it when saved
//then when we load we also save the load files name
fn main() -> ggez::GameResult {
    loop {
        let mut user_game_save_choise = String::new();
        println!("1: New Game \n2: Load Game");

        std::io::stdin()
            .read_line(&mut user_game_save_choise)
            .expect("failed line read");

        let user_game_save_choise: u32 = match user_game_save_choise.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("incorrect input: not a number");
                continue;
            }
        };

        let choise = match user_game_save_choise {
            1 => serialisation::GameOptions::NewGame,
            2 => {
                println!("name of the save file? ");
                let mut input_name: String = String::new();
                std::io::stdin()
                    .read_line(&mut input_name)
                    .expect("failed line read");
                serialisation::GameOptions::LoadGame(input_name.trim().to_string())
            }

            _ => {
                println!("{} is incorrect input", user_game_save_choise);
                continue;
            }
        };

        let state: Option<MainState> = match choise {
            GameOptions::NewGame => Some(MainState::new()),
            GameOptions::LoadGame(filename) => serialisation::load_game(&filename),
        };

        match state {
            Some(state) => {
                //ctx used to be mut, idk if needed. started complaining
                let (ctx, event_loop) = ggez::ContextBuilder::new("gametest", "kimierik")
                    .window_mode(
                        ggez::conf::WindowMode::default()
                            .dimensions(GAME_SCREENW + UIX, GAME_SCREENY + UIY),
                    )
                    .build()
                    .expect("cb ERROR");
                ggez::event::run(ctx, event_loop, state)
            }
            None => continue,
        }
    }
}
