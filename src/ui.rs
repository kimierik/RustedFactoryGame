
use crate::egui;
use crate::game_state::MainState;
use crate::serialisation;

pub fn handle_egui_widget(game:&mut MainState,ctx:&mut ggez::Context){

		let egui_ctx = game.egui_backend.ctx();

		egui::Window::new("menu widget").default_pos([crate::GAME_SCREENW,300.0]).show(&egui_ctx, |ui| {
			ui.label(" ");

			if ui.button("start new game").clicked() {
                let state=MainState::new();
                game.hotload_data(state);
			}
			if ui.button("save game").clicked() {
                serialisation::save_game(&game);
			}
			if ui.button("load game").clicked() {
                let state=serialisation::load_game("test_save.json");
                game.hotload_data(state.unwrap());
			}

			if ui.button("quit").clicked() {
				ctx.request_quit();
			}
		});

		game.egui_backend.update(ctx);
}


