use macroquad::prelude::*;

#[macroquad::main("Hello-World")]
async fn main() {
	loop {
		clear_background(BLACK);
		next_frame().await	
	}
}
