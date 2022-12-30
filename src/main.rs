use std::time::Duration;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color};

mod game_state;
mod inputs;
mod drawables_trait;
//cordinate and input does not need to be in game state
//lets do propper implementation

use game_state::MainState;
use drawables_trait::MakeDrawable;

const GAME_SCREENW: f32 = 600.0;
const GAME_SCREENY: f32 = 600.0;
const UIX: f32 = 300.0;
const UIY: f32 = 300.0;
const KEYBIND_FILENAME:&str="KeyboardInputActions.cfg" ;

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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        //^clears the screen with white

        //self.draw_map(&mut canvas, ctx)?;
        self.draw_object(&mut canvas, ctx, self.get_screen_info())?;

        //draw player
        self.get_player_ref().draw_object(&mut canvas, ctx, self.get_screen_info())?;

        //draw the bg for the ui
        drawables_trait::draw_ui_bg(&mut canvas, ctx)?;

        //draw resources
        canvas.draw(
            &self.get_recource_drawable(),
            graphics::DrawParam::default().dest([GAME_SCREENW, 10.0]),
        );


        //vv puts everything we just draw to the ctx
        canvas.finish(ctx)
    }
}



fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("gametest", "kimierik")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(GAME_SCREENW + UIX, GAME_SCREENY + UIY),
        )
        .build()
        .expect("cb ERROR");

    let state = MainState::new(&mut ctx);
    ggez::event::run(ctx, event_loop, state)
}
