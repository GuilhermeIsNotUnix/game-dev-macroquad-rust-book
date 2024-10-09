use macroquad::prelude::*;

#[macroquad::main("Fly Away")]
async fn main() {
	let mut ball_x = screen_width() / 2.0;
	let mut ball_y = screen_height() / 2.0;
	    	
    loop {    	
		clear_background(BLACK);
		
    	if is_key_down(KeyCode::Right) {
    		ball_x += 1.0;
    	}
    	if is_key_down(KeyCode::Left) {
    		ball_x -= 1.0;
    	}
    	if is_key_down(KeyCode::Up) {
    		ball_y -= 1.0;
    	}
    	if is_key_down(KeyCode::Down) {
    		ball_y += 1.0;
    	}

    	draw_circle(ball_x, ball_y, 16.0, YELLOW);
    	next_frame().await
    }
}
