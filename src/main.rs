use game_state::buildings::material::MaterialValue;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, DrawParam};
use std::time::Duration;
use strum::IntoEnumIterator;


use ggez_egui::egui;

mod drawables_trait;
mod game_state;
mod inputs;
mod serialisation;
mod ui;
//cordinate and input does not need to be in game state
//lets do propper implementation

use drawables_trait::MakeDrawable;
use game_state::buildings::state::State;
use game_state::MainState;
use game_state::buildings::material::Material;


const GAME_SCREENW: f32 = 600.0;
const GAME_SCREENY: f32 = 600.0;
const UIX: f32 = 300.0;
const UIY: f32 = 300.0;
const KEYBIND_FILENAME: &str = "KeyboardInputActions.cfg";

impl EventHandler<ggez::GameError> for game_state::MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        //handle keyboard inputs. and do appropriate reactions
        inputs::handle_keyboard_inputs(self, ctx);


        //calculate production here
        //how mutch rn, how mutch after
        //we can loop to make some vec with appropriate data i think
        //do loop through all tiles and make money out of them
        if self.get_time_since_collect().elapsed() > Duration::from_secs(1) {
           let before=self.get_resource().get_permanent_resources().get_current_materials(); 

            self.loop_tiles_and_apply_effect();
            self.reset_time_since_collect();

           let after=self.get_resource().get_permanent_resources().get_current_materials(); 

           //compare before and after vectors MaterialValues and get difference

           let mut ret:Vec<(Material,MaterialValue)>=vec![];
           for (index,(mat,val)) in before.iter().enumerate(){
               let matval =after[index].1 -val.clone();
               ret.push((mat.clone(),matval));
           }
           self.get_mut_resource().set_income_db(ret);

          
        }



        //handle widget interactions
        ui::handle_egui_widget(self,ctx);


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

        canvas.draw(&self.egui_backend, DrawParam::default());

        //vv puts everything we just draw to the ctx
        canvas.finish(ctx)
    }
}

//TODO
//name the save
fn main() {

    let mut state =MainState::new();
    let (ctx, event_loop) = ggez::ContextBuilder::new("gametest", "kimierik")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(GAME_SCREENW + UIX, GAME_SCREENY + UIY),
        )
        .build()
        .expect("cb ERROR");
 
    state.egui_backend.input.set_scale_factor(1.5, (GAME_SCREENW+UIX,GAME_SCREENY+UIY));
    ggez::event::run(ctx, event_loop, state)
}
