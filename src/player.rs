use ggez::{glam::Vec2, graphics::{Image, ImageFormat}, input::keyboard::KeyCode, Context};
use image::{imageops::overlay, DynamicImage, GenericImage, Rgb, Rgba, RgbaImage};

fn load_file(p: &str) -> Option<Vec<u8>> {
    use std::io::Read as _;
    let path = std::path::PathBuf::from(format!("{p}"));

    if !path.exists() {
        println!("Path doesn't exists: {path:?}");
        return None;
    }

    let mut file = std::fs::File::open(path).ok()?;

    let mut bytes: Vec<u8> = Vec::new();
    let _bytes_read = file.read_to_end(&mut bytes);

    Some(bytes)
}

fn make_test_sprite(ctx: &mut Context) -> Image {
    let original_image = image::open("./assets/icon.png").unwrap();
    let blurred_image = original_image.blur(1.5);

    let mut blurred_image_rgba = dynamic_image_to_rgba_image(blurred_image);
    tint_image(&mut blurred_image_rgba, Rgb([0.0, 1.0, 1.0]));
    let mut combined_image = blurred_image_rgba;
    overlay(&mut combined_image, &original_image, 0, 0);

    let pixels: Vec<u8> = combined_image
        .pixels()
        .flat_map(|pixel| pixel.0.iter().cloned())
        .collect();

    let format = ImageFormat::Rgba8Unorm;
    Image::from_pixels(ctx, &pixels, format, combined_image.width(), combined_image.height())
}

    // let what_the_fuck = combined_image.into_vec();
    // let asd = combined_image.into_raw();

    // let mut bytes: Vec<u8> = Vec::new();
    // let content = image.read_to_end(&mut bytes);
    // let mut content = combined_image.as_raw();
    // let bytes: &[u8] = combined_image.as_raw();

    // let bytes: Vec<u8> = combined_image.clone().into_raw()/;
    
    // Create image from bytes
 
    // let _ = blurred_image_rgba.save("./assets/baked/player.png");

    // // let mut new_image = DynamicImage::new_rgba8(original_image.width(), original_image.height());
    // let mut new_image = blurred_image.clone();
    // // overlay.copy_from(original_image, 0, 0).expect("Failed to copy original image");
    // overlay(&mut new_image, &original_image, 0, 0);

    // let _ = rgba_image.save("./assets/test.png");
    // let mut overlayed_image = original_image.clone();
    
    // draw_image(&mut overlayed_image, &blurred_image, 0, 0, blend);
    // let blurred_image = imageops::blur(&asset_path, 10.0);


    // let asd = 
    // let path = "/icon.png";
    // let image = image::open(path)?;
    
    // Image::from_path(ctx, "/baked/player.png").unwrap()
    // Image::from_path(ctx, "/icon_blurred_overlay.png").unwrap()
// }



pub fn tint_image(image: &mut RgbaImage, tint: Rgb<f32>) {
    let Rgb([tint_r, tint_g, tint_b]) = tint;
    for Rgba([r, g, b, _]) in image.pixels_mut() {
        *r = (*r as f32 * tint_r) as u8;
        *g = (*g as f32 * tint_g) as u8;
        *b = (*b as f32 * tint_b) as u8;
    }
}

fn dynamic_image_to_rgba_image(dynamic_image: DynamicImage) -> RgbaImage {
    // Ensure the DynamicImage is in RGBA format
    let rgba_image = dynamic_image.to_rgba8();

    // Get the dimensions of the image
    let (width, height) = rgba_image.dimensions();

    // Create a new RgbaImage with the same dimensions
    let mut rgba_image_buffer = RgbaImage::new(width, height);

    // Copy the pixel data from the DynamicImage to the RgbaImage
    rgba_image_buffer.copy_from(&rgba_image, 0, 0).expect("Failed to copy pixel data");

    rgba_image_buffer
}

pub struct Player {
	pub sprite: Image,
	pub pos: Vec2,
	pub rotation: f32,
	pub movement_speed: f32,
}

impl Player {
	pub fn new(ctx: &mut Context) -> Player {
		Player {
			sprite: make_test_sprite(ctx),
			// sprite: Image::from_path(ctx, "/icon.png").unwrap(),
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
