use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
				let hi = ram.read_byte(self.pc) as u16;
				let lo = ram.read_byte(self.pc+1) as u16;
				let instruction:u16 = (hi <<8) | lo;
				println!("inst: {:#X} hi: {:#X} lo: {:#X}", instruction, hi, lo);

				let nnn = instruction & 0x0FFF;
				let nn  = instruction & 0x00FF;
				let n   = instruction & 0x000F;
				let x   = (instruction & 0x0F00) >> 8;
				let y   = (instruction & 0x00F0) >> 4;
				
				println!("nnn = {:?} nn = {:?} n = {:?} x = {:?} y = {:?}", nnn, nn, n, x, y);
				match (instruction & 0xF000) >> 12 {
						0x1 => {
								self.pc = nnn;
						}
						_ => panic!("Unrecognized Instruction {:#X}:{:#X}", self.pc, instruction)
						
				}
		}
}
