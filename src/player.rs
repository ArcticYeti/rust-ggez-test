use ggez::{glam::Vec2, graphics::Image, input::keyboard::KeyCode, Context};

pub struct Player {
	pub sprite: Image,
	pub pos: Vec2,
	pub rotation: f32,
	pub movement_speed: f32,
}

impl Player {
	pub fn new(ctx: &mut Context) -> Player {
		Player {
			sprite: Image::from_path(ctx, "/icon.png").unwrap(),
			pos: Vec2::new(0.0, 0.0),
			rotation: 0.0,
			movement_speed: 300.0,
		}
	}

	pub fn update_movement(&mut self, ctx: &mut Context, delta_time_s: f32) {
		let k_ctx = &ctx.keyboard;

		let mut dx: f32 = 0.0;
		let mut dy: f32 = 0.0;

		if k_ctx.is_key_pressed(KeyCode::W) { 
			dy -= 1.0;
		}
		if k_ctx.is_key_pressed(KeyCode::A) {
			dx -= 1.0;
		}
		if k_ctx.is_key_pressed(KeyCode::S) {
			dy += 1.0;
		}
		if k_ctx.is_key_pressed(KeyCode::D) {
			dx += 1.0;
		}

		if dx != 0.0 && dy != 0.0 {
			let length = (dx * dx + dy * dy).sqrt();
			dx /= length;
			dy /= length;
		}

		self.pos[0] += dx * self.movement_speed * delta_time_s;
		self.pos[1] += dy * self.movement_speed * delta_time_s;
	}
	pub fn rotate_facing(&mut self, target_pos: Vec2) {
		let direction = target_pos - self.pos;
		let angle = direction.y.atan2(direction.x);
		self.rotation = angle;
	}
}
