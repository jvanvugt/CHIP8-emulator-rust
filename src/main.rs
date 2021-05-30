use std::env;

enum Opcode {
    CallMachineCode(u16),
    Clear,
    Return,
    Goto(u16),
    Call(u16),
    Eq(u8, u8),
    Neq(u8, u8),
    EqReg(u8, u8),
    Set(u8, u8),
    AddConst(u8, u8),
    SetReg(u8, u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    Shr(u8),
    SubSwapped(u8, u8),
    Shl(u8),
    NeqReg(u8, u8),
    SetI(u16),
    Jump(u16),
    Rand(u8, u8),
    Draw(u8, u8, u8),
    Key(u8),
    Nkey(u8),
    GetDelay(u8),
    WaitKey(u8),
    SetDelay(u8),
    SetSound(u8),
    AddI(u8),
    SetISprite(u8),
    SetBCD(u8),
    DumpReg(u8),
    LoadReg(u8),
}

struct CHIP8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 24],
    delay_timer: u32,
    sound_timer: u32,
    i_reg: u16,
    pc: u16,
    program: Vec<u16>,
}

impl CHIP8 {
    fn new(program: Vec<u16>) -> CHIP8 {
        CHIP8 {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 24],
            delay_timer: 0,
            sound_timer: 0,
            i_reg: 0,
            pc: 0,
            program: program
        }
    }

    fn execute(&self) {
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected exactly 2 arguments");
        return
    }
    let rom_file = &args[1];
    let contents = std::fs::read(rom_file).unwrap();
    let program: Vec<u16> = contents.chunks(2).into_iter().map(|x| ((x[0] as u16) << 8) | (x[1] as u16)).collect();
    let chip8 = CHIP8::new(program);
    chip8.execute();
}
