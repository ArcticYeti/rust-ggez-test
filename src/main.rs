mod player;

use std::{path::PathBuf, time::Duration};
use ggez::{
	conf, event::{self, EventHandler}, glam::Vec2, graphics, winit::window::WindowButtons, Context, ContextBuilder, GameError, GameResult
};

use player::Player;

struct MyGame {
	delta_time: Duration,
	player_instance: Player,
}

impl MyGame {
	fn new(ctx: &mut Context) -> GameResult<MyGame> {
		let s = MyGame {
			delta_time: Duration::new(0, 0),
			player_instance: Player::new(ctx),
		};

		Ok(s)
	}
}

impl EventHandler<GameError> for MyGame {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.delta_time = ctx.time.delta();
		let delta_time_s = self.delta_time.as_secs_f32();

		let m_ctx = &ctx.mouse;
		let mouse_pos = m_ctx.position();
		let mouse_pos_vec2 = Vec2::new(mouse_pos.x, mouse_pos.y);

		self.player_instance.update_movement(ctx, delta_time_s);
		self.player_instance.rotate_facing(mouse_pos_vec2);

		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas =
			graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

		let draw_params = graphics::DrawParam::new()
			.dest(self.player_instance.pos)
			.rotation(self.player_instance.rotation)
			.offset(Vec2::new(0.5, 0.5));

		canvas.draw(&self.player_instance.sprite, draw_params);
		canvas.finish(ctx)?;

		// println!("delta_time = {}ns", self.dt.as_nanos());
		Ok(())
	}
}

pub fn main() {
	let assets_dir = PathBuf::from("./assets");

	let c = conf::Conf::new();
	let (mut ctx, event_loop) = ContextBuilder::new("glowy", "SilentDreamer")
		.default_conf(c)
		.add_resource_path(assets_dir)
		.window_setup(conf::WindowSetup::default()
			.title("Glowy!"))
		.window_mode(conf::WindowMode::default()
			.dimensions(1280.0, 720.0)
			.resizable(false))
		.build()
		.unwrap();

	let window_ref = ctx.gfx.window();
	let mut buttons = WindowButtons::all();
	buttons.remove(WindowButtons::MAXIMIZE);
	window_ref.set_enabled_buttons(buttons);

	let my_game = MyGame::new(&mut ctx).unwrap();
	event::run(ctx, event_loop, my_game);
}
