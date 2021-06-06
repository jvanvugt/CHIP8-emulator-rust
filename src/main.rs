use rand;
use std::env;
use sdl2;

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: u8 = 16;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const FONT_DATA: [u8; 16*5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
    0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
    0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
    0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
    0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

struct CHIP8 {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; NUM_REGISTERS],
    stack: [u16; STACK_SIZE as usize],
    delay_timer: u8,
    sound_timer: u8,
    i_reg: u16,
    pc: u16,
    sp: u8,
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl CHIP8 {
    fn new(program: &Vec<u8>) -> CHIP8 {
        let program_start: usize = 0x200;
        let mut chip8 = CHIP8 {
            memory: [0; MEMORY_SIZE],
            registers: [0; NUM_REGISTERS],
            stack: [0; STACK_SIZE as usize],
            delay_timer: 0,
            sound_timer: 0,
            i_reg: 0,
            pc: program_start as u16,
            sp: 0,
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        };
        chip8.init_font();
        for (i, byte) in program.iter().enumerate() {
            chip8.memory[program_start + i] = *byte;
        }
        chip8
    }

    fn init_font(&mut self) {
        for (i, byte) in FONT_DATA.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }
    fn draw_screen(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                print!("{}", if self.screen[y][x] { "#" } else { " " });
            }
            print!("\n");
        }
        print!("\n");
    }

    fn execute_op(&mut self, opcode: u16) {
        let a = opcode >> 12 as u8;
        let b = ((opcode >> 8) & 0xf) as u8;
        let c = ((opcode >> 4) & 0xf) as u8;
        let d = ((opcode >> 0) & 0xf) as u8;
        match (a, b, c, d) {
                // CLS
                (0, 0, 0xE, 0) => {
                    self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
                    self.pc += 2;
                    self.draw_screen();
                }
                ,
                // 00EE - RET
                (0, 0, 0xE, 0xE) => {
                    if self.sp == 0 {
                        std::process::exit(0);
                    }
                    self.pc = self.stack[(self.sp - 1) as usize];
                    self.sp -= 1;
                },
                // SYS addr
                (0, _, _, _) => panic!("Machine code call not supported"),
                // 1nnn - JP addr
                (1, _, _, _) => self.pc = opcode & 0x0fff,
                // 2nnn - CALL addr
                (2, _, _, _) => {
                    assert!(self.sp < STACK_SIZE);
                    self.stack[self.sp as usize] = self.pc + 2;
                    self.sp += 1;
                    self.pc = opcode & 0x0fff;
                },
                // 3xkk - SE Vx, byte
                (3, _, _, _) => {
                    if self.registers[b as usize] == (c << 4) | d {
                        self.pc += 2;
                    }
                    self.pc += 2;
                },
                // 4xkk - SNE Vx, byte
                (4, _, _, _) => {
                    if self.registers[b as usize] != (c << 4) | d {
                        self.pc += 2;
                    }
                    self.pc += 2;
                },
                // 5xy0 - SE Vx, Vy
                (5, _, _, 0) => {
                    if self.registers[b as usize] == self.registers[c as usize] {
                        self.pc += 2;
                    }
                    self.pc += 2;
                },
                // 6xkk - LD Vx, byte
                (6, _, _, _) => {
                    self.registers[b as usize] = (c << 4) | d;
                    self.pc += 2;
                },
                // 7xkk - ADD Vx, byte
                (7, _, _, _) => {
                    let vx = self.registers[b as usize];
                    self.registers[b as usize] = vx.overflowing_add((c << 4) | d).0;
                    self.pc += 2;
                },
                // 8xy0 - LD Vx, Vy
                (8, _, _, 0) => {
                    self.registers[b as usize] = self.registers[c as usize];
                    self.pc += 2;
                },
                // 8xy1 - OR Vx, Vy
                (8, _, _, 1) => {
                    self.registers[b as usize] |= self.registers[c as usize];
                    self.pc += 2;
                },
                // 8xy2 - AND Vx, Vy
                (8, _, _, 2) => {
                    self.registers[b as usize] &= self.registers[c as usize];
                    self.pc += 2;
                },
                // 8xy3 - XOR Vx, Vy
                (8, _, _, 3) => {
                    self.registers[b as usize] ^= self.registers[c as usize];
                    self.pc += 2;
                },
                // 8xy4 - ADD Vx, Vy
                (8, _, _, 4) => {
                    let vx = self.registers[b as usize];
                    let (result, overflow) = vx.overflowing_add(self.registers[c as usize]);
                    self.registers[0xf] = overflow as u8;
                    self.registers[b as usize] = result;
                    self.pc += 2;
                },
                // 8xy5 - SUB Vx, Vy
                (8, _, _, 5) => {
                    let vx = self.registers[b as usize];
                    let (result, overflow) = vx.overflowing_sub(self.registers[c as usize]);
                    self.registers[0xf] = overflow as u8;
                    self.registers[b as usize] = result;
                    self.pc += 2;
                },
                // 8xy6 - SHR Vx {, Vy}
                (8, _, _, 6) => {
                    self.registers[0xf] = self.registers[b as usize] & 1;
                    self.registers[b as usize] >>= 1;
                    self.pc += 2;
                },
                // 8xy7 - SUBN Vx, Vy
                (8, _, _, 7) => {
                    self.registers[0xf] = (self.registers[c as usize] > self.registers[b as usize]) as u8;
                    self.registers[c as usize] -= self.registers[b as usize];
                    self.pc += 2;
                },
                // 8xyE - SHL Vx {, Vy}
                (8, _, _, 0xE) => {
                    self.registers[0xf] = self.registers[b as usize] >> 7;
                    self.registers[b as usize] <<= 1;
                    self.pc += 2;
                },
                // 9xy0 - SNE Vx, Vy
                (9, _, _, 0) => {
                    if self.registers[b as usize] != self.registers[c as usize] {
                        self.pc += 2;
                    }
                    self.pc += 2;
                },
                // Annn - LD I, addr
                (0xA, _, _, _) => {
                    self.i_reg = opcode & 0x0fff;
                    self.pc += 2;
                },
                // Bnnn - JP V0, addr
                (0xB, _, _, _) => {
                    self.pc = (self.registers[0] as u16) + (opcode & 0x0fff);
                },
                // Cxkk - RND Vx, byte
                (0xC, _, _, _) => {
                    self.registers[b as usize] = rand::random::<u8>() & ((c << 4) | d);
                    self.pc += 2;
                },
                // Dxyn - DRW Vx, Vy, nibble
                (0xD, _, _, _) => {
                    let start_x = self.registers[b as usize] as usize;
                    let start_y = self.registers[c as usize] as usize;
                    self.registers[0xf] = 0;
                    for y in 0..(d) {
                        let row_byte = self.memory[(self.i_reg + y as u16) as usize];
                        let mut y_loc = y as usize + start_y;
                        if y_loc >= SCREEN_HEIGHT {
                            y_loc -= SCREEN_HEIGHT;
                        }
                        for x in 0..8 {
                            let mut x_loc = x as usize + start_x;
                            if x_loc >= SCREEN_WIDTH {
                                x_loc -= SCREEN_WIDTH;
                            }
                            let color = (row_byte >> (7 - x)) & 1 == 1;
                            if self.screen[y_loc][x_loc] && color {
                                self.registers[0xf] = 1;
                            }
                            self.screen[y_loc][x_loc] ^= color;
                        }
                    }
                    self.pc += 2;
                    self.draw_screen();
                },
                // Ex9E - SKP Vx
                (0xE, _, 0x9, 0xE) => {
                    panic!("Key press not yet implemented");
                },
                // ExA1 - SKNP Vx
                (0xE, _, 0xA, 0x1) => {
                    panic!("Key press not yet implemented");
                },
                // Fx07 - LD Vx, DT
                (0xF, _, 0x0, 0x7) => {
                    self.registers[b as usize] = self.delay_timer;
                    self.pc += 2;
                },
                // Fx0A - LD Vx, K
                (0xF, _, 0x0, 0xA) => {
                    panic!("Key press not yet implemented");
                },
                // Fx15 - LD DT, Vx
                (0xF, _, 0x1, 0x5) => {
                    self.delay_timer = self.registers[b as usize];
                    self.pc += 2;
                },
                // Fx18 - LD ST, Vx
                (0xF, _, 0x1, 0x8) => {
                    self.sound_timer = self.registers[b as usize];
                    self.pc += 2;
                },
                // Fx1E - ADD I, Vx
                (0xF, _, 0x1, 0xE) => {
                    self.i_reg += self.registers[b as usize] as u16;
                    self.pc += 2;
                },
                // Fx29 - LD F, Vx
                (0xF, _, 0x2, 0x9) => {
                    let digit = self.registers[b as usize] as u16;
                    assert!(digit <= 0xF);
                    self.i_reg = 5 * digit;
                    self.pc += 2;
                },
                // Fx33 - LD B, Vx
                (0xF, _, 0x3, 0x3) => {
                    let digit = self.registers[b as usize];
                    self.memory[self.i_reg as usize] = digit / 100;
                    self.memory[(self.i_reg + 1) as usize] = (digit % 100) / 10;
                    self.memory[(self.i_reg + 2) as usize] = digit % 10;
                    self.pc += 2;
                },
                // Fx55 - LD [I], Vx
                (0xF, _, 0x5, 0x5) => {
                    for x in 0..(b+1) {
                        self.memory[(self.i_reg + x as u16) as usize] = self.registers[x as usize];
                    }
                    self.pc += 2;
                },
                // Fx65 - LD Vx, [I]
                (0xF, _, 0x6, 0x5) => {
                    for x in 0..(b+1) {
                        self.registers[x as usize] = self.memory[(self.i_reg + x as u16) as usize];
                    }
                    self.pc += 2;
                },
                _ => panic!("Unknown opcode 0x{:0X}{:0X}{:0X}{:0X}", a, b, c, d),
            };
    }

    fn execute(&mut self) {
        loop {
            let high_byte = self.memory[self.pc as usize] as u16;
            let low_byte = self.memory[(self.pc + 1) as usize] as u16;
            self.execute_op((high_byte << 8) | low_byte);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected exactly 2 arguments");
        return;
    }
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let _window = video_subsystem.window("CHIP-8 Emulator", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let rom_file = &args[1];
    let rom_contents = std::fs::read(rom_file).unwrap();
    let mut chip8 = CHIP8::new(&rom_contents);
    chip8.execute();
}
