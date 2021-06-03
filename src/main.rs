use std::env;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
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

fn parse_opcode(opcode: u16) -> Instruction {
    use Instruction::*;
    let a = opcode >> 12;
    let b = (opcode >> 8) & 0xf;
    let c = (opcode >> 4) & 0xf;
    let d = (opcode >> 0) & 0xf;
    if a == 0 {
        if b == 0 {
            assert!(c==0xe);
            if d == 0 {
                return Clear;
            }
            if d == 0xe {
                return Return;
            }
        }
        return CallMachineCode(opcode & 0x0fff);
    }
    if a == 1 {
        return Goto(opcode & 0x0fff);
    }
    if a == 2 {
        return Call(opcode & 0x0fff);
    }

    panic!("Unknown opcode {} {} {} {}", a, b, c, d);
}

fn parse_program(program: &Vec<u16>) -> Vec<Instruction> {
    program.iter().map(|p| parse_opcode(*p)).collect()
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
    screen: [[bool; 64]; 32],
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
            program: program,
            screen: [[false; 64]; 32],
        }
    }

    fn execute(&mut self) {
        let instructions = parse_program(&self.program);
        while (self.pc as usize) < self.program.len() {
            let instr = instructions[self.pc as usize];
            match  instr {
                _ => panic!("Instruction {:?} not yet implemented", instr),
            }
        }
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
    let mut chip8 = CHIP8::new(program);
    chip8.execute();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode(0x0fff), Instruction::CallMachineCode(0xfff));
        assert_eq!(parse_opcode(0x00e0), Instruction::Clear);
        assert_eq!(parse_opcode(0x00ee), Instruction::Return);
        assert_eq!(parse_opcode(0x1fff), Instruction::Goto(0xfff));
        assert_eq!(parse_opcode(0x2fff), Instruction::Call(0xfff));
    }
}