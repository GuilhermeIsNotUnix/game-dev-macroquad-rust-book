use macroquad::prelude::*;

#[macroquad::main("Smooth Movement")]
async fn main() {
	const MOVEMENT_SPEED: f32 = 200.0;
	
	let mut ball_x = screen_width() / 2.0;
	let mut ball_y = screen_height() / 2.0;

	loop {
		clear_background(BLACK);
		let delta_time = get_frame_time();

		if is_key_down(KeyCode::Right) {
			ball_x += MOVEMENT_SPEED * delta_time;
		}

		if is_key_down(KeyCode::Left) {
			ball_x -= MOVEMENT_SPEED * delta_time;
		}

		if is_key_down(KeyCode::Up) {
			ball_y -= MOVEMENT_SPEED * delta_time;
		}

		if is_key_down(KeyCode::Down) {
			ball_y += MOVEMENT_SPEED * delta_time;
		}

		ball_x = clamp(ball_x, 0.0, screen_width());
		ball_y = clamp(ball_y, 0.0, screen_height());

		draw_circle(ball_x, ball_y, 16.0, YELLOW);
		next_frame().await
	}
}
