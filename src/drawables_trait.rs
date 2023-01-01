use ggez::graphics;
use ggez::{self, graphics::Canvas};

use crate::game_state::screen_info::ScreenInfo;
use crate::game_state::cordinate::Cordinates;
use crate::GAME_SCREENW;

//ugly draw ui function that is not in main
pub fn draw_ui_bg(canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult {
    let game_space = graphics::Rect::new(
        GAME_SCREENW,
        0.0,
        crate::UIX,
        crate::GAME_SCREENY + crate::UIY,
    );
    let game_space_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        game_space,
        graphics::Color::BLACK,
    )?;
    canvas.draw(&game_space_mesh, graphics::DrawParam::default());

    let game_space_b =
        graphics::Rect::new(0.0, crate::GAME_SCREENY, crate::GAME_SCREENW, crate::UIY);
    let game_space_mesh_b = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        game_space_b,
        graphics::Color::BLACK,
    )?;
    canvas.draw(&game_space_mesh_b, graphics::DrawParam::default());
    Ok(())
}



pub fn make_rect(location:Cordinates,w:f32,h:f32,col:graphics::Color,ctx: &mut ggez::Context,canvas: &mut Canvas)->ggez::GameResult{
    let game_space_b =
        graphics::Rect::new(location.x, location.y, w, h);

    let game_space_mesh_b = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        game_space_b,
        col,
    )?;
    canvas.draw(&game_space_mesh_b, graphics::DrawParam::default());

    Ok(())
}




//wanted to try traits
pub trait MakeDrawable {
    fn draw_object(
        &self,
        _canvas: &mut Canvas,
        _ctx: &mut ggez::Context,
        _screendata: &ScreenInfo,
    ) -> ggez::GameResult {
        Ok(())
    }
}

impl MakeDrawable for super::MainState {
    fn draw_object(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        _screendata: &ScreenInfo,
    ) -> ggez::GameResult {
        for item in self.get_map().iter() {
            let mesh = item.get_drawable(ctx, self.get_screen_info())?;
            canvas.draw(&mesh, ggez::graphics::DrawParam::default());
        }
        Ok(())
    }
}

impl MakeDrawable for super::game_state::player::Player {
    fn draw_object(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        screen: &ScreenInfo,
    ) -> ggez::GameResult {
        let world_cords = self.get_cords().world_to_screen(screen);
        let p = ggez::graphics::Rect::new(
            world_cords.x,
            world_cords.y,
            screen.get_tile_size(),
            screen.get_tile_size(),
        );
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            p,
            graphics::Color::RED,
        )?;
        canvas.draw(&mesh, ggez::graphics::DrawParam::default());
        Ok(())
    }
}
