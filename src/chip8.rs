use std::fs;
use rand;
use super::{WIDTH,HEIGHT};

const FONT_START: usize = 0x050;
 
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8{
    mem:[u8;4096],
    i:u16,
    pc:u16,
    pub display:[[bool;64];32],
    v:[u8;16],
    stack:Vec<u16>,
    pub keys:[bool;16],
    delay:u8,
    sound:u8,
    pub released_key:Option<u8>
}
impl Chip8{
    pub fn new(path:&str)->Self{
        let rom = fs::read(path).expect("Could not load rom");
        let mut mem = [0u8;4096];
        for i in 0..rom.len(){
            mem[0x200 + i] = rom[i]
        }
        for i in 0..FONT.len(){
            mem[FONT_START + i] = FONT[i];
        }
        return Chip8 { mem:mem, i: 0,pc:0x200, display:[[false;64];32],v:[0;16],stack:Vec::new(),keys:[false;16],delay:0,sound:0,released_key:None};
    }
    fn fetch(&mut self)->u16{
        let op:u16 = ((self.mem[self.pc as usize] as u16) << 8) | self.mem[self.pc as usize + 1] as u16;
        self.pc += 2;
        return op; 
    }
    pub fn display_command(&mut self, mut sx:u8,mut sy:u8,n:u8){
        self.v[0xF] = 0;
        sx = sx%WIDTH as u8;
        sy = sy%HEIGHT as u8;
        for i in 0..n{
            let y = (sy + i) as usize;
            if y >= 32{
                break;
            }
            let cur_byte = self.mem[(self.i + i as u16) as usize];
            for j in 0..8{
                let x = (sx + j) as usize;
                if x >= 64{
                    break;
                }
                let bit = (cur_byte >> (7-j))&1 != 0;

                if self.display[y][x] == true && bit == true{
                    self.v[0xF] = 1;
                }
                self.display[y][x] ^= bit;
            }
        }
    }
    fn decode_and_execute(&mut self, op:u16){
        let kind = (op&0xF000) >> 12;
        let x = ((op&0x0F00) >> 8) as usize;
        let y = ((op&0x00F0) >> 4) as usize;
        let n = (op&0x000F) as u8;
        let nn = (op&0x00FF) as u8;
        let nnn = op&0x0FFF;
        match kind{
            0x0 if op == 0x00E0 => self.display = [[false;64];32],
            0x0 if op == 0x00EE => {self.pc = self.stack.pop().expect("popped empty stack")},
            0x1 => self.pc = nnn,
            0x2 => {self.stack.push(self.pc); self.pc = nnn},
            0x3 => {if self.v[x] == nn {self.pc += 2}},
            0x4 => {if self.v[x] != nn {self.pc += 2}},
            0x5 => {if self.v[x] == self.v[y] {self.pc += 2}},
            0x6 => self.v[x] = nn,
            0x7 => self.v[x] = self.v[x].wrapping_add(nn),
            0x8 => {
                match n{
                    0x0 => self.v[x] = self.v[y],
                    0x1 => self.v[x] |= self.v[y],
                    0x2 => self.v[x] &= self.v[y],
                    0x3 => self.v[x] ^= self.v[y],
                    0x4 => {
                        let (r,c) = self.v[x].overflowing_add(self.v[y]);
                        self.v[x] = r; self.v[0xF] = c as u8;
                    },
                    0x5 => {
                        let (r,c) = self.v[x].overflowing_sub(self.v[y]);
                        self.v[x] = r; self.v[0xF] = (!c) as u8;
                    },
                    0x6 => {let (r,c) = self.v[x].overflowing_shl(1); self.v[x] = r; self.v[0xF] = c as u8;}
                    0x7 => {
                        let (r,c) = self.v[y].overflowing_sub(self.v[x]);
                        self.v[x] = r; self.v[0xF] = (!c) as u8;
                    },
                    0xE => {let (r,c) = self.v[x].overflowing_shr(1); self.v[x] = r; self.v[0xF] = c as u8;},
                    _ => println!("unexpected command {}", op),
                }
            }
            0x9 => {if self.v[x] != self.v[y] {self.pc += 2}},
            0xA => self.i = nnn,
            0xB => self.i = self.v[0x0] as u16 + nnn,
            0xC => self.v[x] = rand::random::<u8>()&nn,
            0xD => self.display_command(self.v[x], self.v[y], n),
            0xE => match nn{
                0x9E => {if self.keys[self.v[x] as usize] {self.pc += 2}},
                0xA1 => {if !self.keys[self.v[x] as usize] {self.pc += 2}},
                _ => println!("unknown instruction {}", op)
            }
            0xF => match nn{
                0x07 => self.v[x] = self.delay,
                0x0A => match self.released_key{Some(v) => self.v[x] = v, None => self.pc -= 2},
                0x15 => self.delay = self.v[x],
                0x18 => self.sound = self.v[x],
                0x1E => self.i = self.i.wrapping_add(self.v[x] as u16),
                0x33 => {
                    let fi = self.v[x] / 100;
                    let si = (self.v[x] / 10)%10;
                    let ti = self.v[x]%10;
                    self.mem[self.i as usize] = fi; self.mem[self.i as usize+1] = si; self.mem[self.i as usize +2] = ti;    
                }
                0x55 => {
                    for i in 0..x{
                        self.mem[self.i as usize + i] = self.v[i];
                    }
                }
                0x65 => {
                    for i in 0..x{
                        self.v[i] = self.mem[self.i as usize + i];
                    }
                }
                _ => println!("unknown instruction {}", op)
            }
            _ => println!("unknown instruction {}", kind)
        }
    }
    pub fn step(&mut self){
        let op = self.fetch();
        self.decode_and_execute(op);
        self.released_key = None;
    }
}