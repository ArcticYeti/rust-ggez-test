use std::path::PathBuf;
use ggez::{
    conf, event, glam::{self, Vec2}, graphics, input::keyboard::KeyCode, Context, ContextBuilder, GameError, GameResult
};

type Point2 = glam::Vec2;

struct MainState {
    dt: std::time::Duration,
    image_icon: graphics::Image,
    rotation: f32,
    player_x: f32,
    player_y: f32,
    movement_speed: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image_icon = graphics::Image::from_path(ctx, "/icon.png")?;

        let s = MainState {
            dt: std::time::Duration::new(0, 0),
            image_icon,
            rotation: 0.0,
            player_x: 0.0,
            player_y: 0.0,
            movement_speed: 300.0,
        };

        Ok(s)
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
    
        let k_ctx = &ctx.keyboard;
        let m_ctx = &ctx.mouse;

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

        self.player_x += dx * self.movement_speed * self.dt.as_secs_f32();
        self.player_y += dy * self.movement_speed * self.dt.as_secs_f32();

        let player_position = Vec2::new(self.player_x, self.player_y);
        let mouse_position = m_ctx.position();
        let mouse_vector = Vec2::new(mouse_position.x, mouse_position.y);
        let direction = mouse_vector - player_position;
        let angle = direction.y.atan2(direction.x);

        self.rotation = angle;

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        // Draw player
        let dst = glam::Vec2::new(self.player_x, self.player_y);
        let draw_params = graphics::DrawParam::new()
            .dest(dst)
            .rotation(self.rotation)
            .offset(Point2::new(0.5, 0.5));

        canvas.draw(&self.image_icon, draw_params);
        canvas.finish(ctx)?;

        // println!("delta_time = {}ns", self.dt.as_nanos());
        Ok(())
    }
}

pub fn main() {
    let resource_dir = PathBuf::from("./resources");

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("glowy", "SilentDreamer")
        .default_conf(c)
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Glowy"))
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0))
        .build()
        .unwrap();

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}
