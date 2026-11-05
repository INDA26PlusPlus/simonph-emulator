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
const KEYS:[(KeyCode,usize);16]= [
    (KeyCode::Key2,0x1),(KeyCode::Key3,0x2), (KeyCode::Key4,0x3), (KeyCode::Key5, 0xC),
    (KeyCode::W, 0x4),(KeyCode::E,0x5), (KeyCode::R, 0x6), (KeyCode::T, 0xD),
    (KeyCode::S,0x7),(KeyCode::D,0x8),(KeyCode::F,0x9), (KeyCode::G, 0xE),
    (KeyCode::X,0xA), (KeyCode::C, 0x0), (KeyCode::V, 0xB), (KeyCode::B,0xF)
];
#[macroquad::main(window_conf)]
async fn main() {
    //let file = "chip8-test-suite/bin/1-chip8-logo.ch8";
    let file = "chip8-test-suite/bin/2-ibm-logo.ch8";
    //let file = "chip8-test-suite/bin/3-corax+.ch8";
    //let file= "chip8-test-suite/bin/4-flags.ch8";
    //let file = "chip8-test-suite/bin/5-quirks.ch8";
    //let file = "chip8-test-suite/bin/6-keypad.ch8";
    //let file = "chip8-test-suite/bin/8-scrolling.ch8";
    let mut cpu = chip8::Chip8::new(file);
    let mut acc = 0.0;
    let tick = 1.0/60.0;
    loop{
        
        for (keycode, keyi) in KEYS{
            match (is_key_down(keycode), cpu.keys[keyi]){
                (false,true) => {cpu.keys[keyi] = false; cpu.released_key = Some(keyi as u8);},
                (true, false) => cpu.keys[keyi] = true,
                _ => {}
            }
        }
        acc += get_frame_time();
        while acc >= tick{
            acc -= tick;
            cpu.decrease_timers();
            for _ in 0..11{
                cpu.step();
            }
        }
        let scale = screen_width()/WIDTH as f32;
        clear_background(BLACK);
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
