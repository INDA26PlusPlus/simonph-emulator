mod chip8;

fn main() {
    //let file = "chip8-test-suite/bin/1-chip8-logo.ch8";
    let file = "chip8-test-suite/bin/2-ibm-logo.ch8";
    let mut emulator = chip8::Chip8::new(file);
    emulator.simulate();
}
