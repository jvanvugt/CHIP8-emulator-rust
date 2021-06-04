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
    let a = opcode >> 12 as u8;
    let b = ((opcode >> 8) & 0xf) as u8;
    let c = ((opcode >> 4) & 0xf) as u8;
    let d = ((opcode >> 0) & 0xf) as u8;
    dbg!(a, b, c, d);
    if a == 0 {
        if b == 0 {
            // assert_eq!(c, 0xe);
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
    if a == 3 {
        return Eq(b, (c << 4) | d);
    }
    if a == 4 {
        return Neq(b, (c << 4) | d);
    }
    if a == 5 {
        assert_eq!(d, 0);
        return EqReg(b, c);
    }
    if a == 6 {
        return Set(b, (c << 4) | d);
    }
    if a == 7 {
        return AddConst(b, (c << 4) | d);
    }
    if a == 8 {
        return match d {
            0 => SetReg(b, c),
            1 => Or(b, c),
            2 => And(b, c),
            3 => Xor(b, c),
            4 => Add(b, c),
            5 => Sub(b, c),
            6 => Shr(b),
            7 => SubSwapped(b, c),
            0xe => Shl(b),
            _ => panic!("Unknown last nibble {}", d)
        };
    }
    if a == 9 {
        assert_eq!(d, 0);
        return NeqReg(b, c);
    }
    if a == 0xa {
        return SetI(opcode & 0x0fff);
    }
    if a == 0xb {
        return Jump(opcode & 0x0fff);
    }
    if a == 0xc {
        return Rand(b, (c << 4) | d);
    }
    if a == 0xd {
        return Draw(b, c, d);
    }
    if a == 0xe {
        return match c {
            0x9 => Key(b),
            0xa => Nkey(b),
            _ => panic!("Unknown 3rd nibble {:#0X}", c),
        }
        // return match last_byte {
        //     0x9e => Key(b),
        //     0xa1 => Nkey(b),
        //     _ => panic!("Unknown last byte {:#0X}", last_byte),
        // }
    }
    if a == 0xf {
        let last_byte = (c << 4) | d;
        return match last_byte {
            0x07 => GetDelay(b),
            0x0a => WaitKey(b),
            0x15 => SetDelay(b),
            0x18 => SetSound(b),
            0x1e => AddI(b),
            0x29 => SetISprite(b),
            0x33 => SetBCD(b),
            0x55 => DumpReg(b),
            0x65 => LoadReg(b),
            _ => panic!("Unknown last byte {:#0X}", last_byte),
        }
    }

    panic!("Unknown opcode {} {} {} {}", a, b, c, d);
}

fn parse_program(program: &Vec<u16>) -> Vec<Instruction> {
    program.iter().map(|p| parse_opcode(*p)).collect()
}

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const CALL_STACK_SIZE: usize = 24;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct CHIP8 {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; NUM_REGISTERS],
    stack: [u16; CALL_STACK_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    i_reg: u16,
    pc: u16,
    program: Vec<u16>,
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
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
        for instr in &instructions {
            dbg!(instr);
        }
        while (self.pc as usize) < self.program.len() {
            let instr = instructions[self.pc as usize];
            match instr {
                _ => panic!("Instruction {:?} not yet implemented", instr),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected exactly 2 arguments");
        return;
    }
    let rom_file = &args[1];
    let contents = std::fs::read(rom_file).unwrap();
    let program: Vec<u16> = contents
        .chunks(2)
        .into_iter()
        .map(|x| ((x[0] as u16) << 8) | (x[1] as u16))
        .collect();
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
        assert_eq!(parse_opcode(0x3fff), Instruction::Eq(0xf, 0xff));
    }
}
