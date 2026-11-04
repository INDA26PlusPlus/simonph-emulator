use macroquad::prelude::*;
mod chip8;
const SCALE:f32 = 32.0;
const WIDTH:usize = 64;
const HEIGHT:usize = 32;
// const HEIGHT_OFFSET:f32 = HEIGHT as f32 *SCALE/2.0;
// const WIDTH_OFFSET:f32 = WIDTH as f32*SCALE/2.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "CHIP-8".to_owned(),
        window_width: (WIDTH as f32 * SCALE) as i32,
        window_height: (HEIGHT as f32 * SCALE) as i32,
        window_resizable: true,
        ..Default::default()
   }
}

#[macroquad::main(window_conf)]
async fn main() {
    //let file = "chip8-test-suite/bin/1-chip8-logo.ch8";
    let file = "chip8-test-suite/bin/2-ibm-logo.ch8";
    let mut cpu = chip8::Chip8::new(file);
    loop{
        clear_background(BLACK);
        cpu.step();
        let scale = screen_width()/WIDTH as f32;
        for i in 0..HEIGHT{
            for j in 0..WIDTH{
                if cpu.display[i][j] == true{
                    draw_rectangle(j as f32*scale, i as f32 * scale, scale, scale, WHITE);
                }
            }
        }
        next_frame().await
    }
}
