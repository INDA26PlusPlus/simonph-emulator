use std::fs;
pub struct Chip8{
    mem:[u8;4096],
    i:u16,
    pc:u16,
    display:[[bool;64];32],
    v:[u16;16]
}
impl Chip8{
    pub fn new(path:&str)->Self{
        let rom = fs::read(path).expect("Could not load rom");
        let mut mem = [0u8;4096];
        for i in 0..rom.len(){
            mem[0x200 + i] = rom[i]
        }
        
        return Chip8 { mem:mem, i: 0,pc:0x200, display:[[false;64];32],v:[0;16]};
    }
    fn fetch(&mut self)->u16{
        let op:u16 = ((self.mem[self.pc as usize] as u16) << 8) | self.mem[self.pc as usize + 1] as u16;
        self.pc += 2;
        return op; 
    }
    pub fn display(&mut self){
        let mut print_str = "".to_string();
        print_str.push_str("\x1B[H");
        for i in 0..32{
            for j in 0..64{
                if self.display[i][j] == false{
                    print_str.push_str("  ");
                }
                else{
                    print_str.push_str("██");
                }
            }
            print_str.push_str("\n");
        }
        print!("{print_str}");
    }
    pub fn display_command(&mut self, sx:u16,sy:u16,n:u16){
        self.v[0xF] = 0;
        for i in 0..n{
            let y = (sy + i) as usize;
            if y >= 32{
                break;
            }
            let cur_byte = self.mem[(self.i + i) as usize];
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
        let n = op&0x000F;
        let nn = op&0x00FF;
        let nnn = op&0x0FFF;
        match kind{
            0x0 if op == 0x00E0 => self.display = [[false;64];32],
            0x1 => self.pc = nnn,
            0x6 => self.v[x] = nn,
            0x7 => self.v[x] += nn,
            0xA => self.i = nnn,
            0xD => self.display_command(self.v[x], self.v[y], n),
            _ => println!("not implemented {}", kind)
        }
    }
    pub fn simulate(&mut self){
        loop{
            let op = self.fetch();
            self.decode_and_execute(op);
            self.display();
        }
    }
}