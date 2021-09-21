use std::fmt;

use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

#[allow(dead_code)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
		prev_pc: u16,
		ret_stack: Vec<u16>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
						prev_pc: 0,
						ret_stack: Vec::<u16>::new(),
        }
    }

		pub fn write_reg_vx(&mut self, index:u8, value:u8) {
				self.vx[index as usize] = value;
		}

		#[allow(dead_code)]		
		pub fn read_reg_vx(&mut self, index:u8) -> u8 {
		    self.vx[index as usize]
		}

		pub fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y:u8, height: u8) {
				println!("Drawing sprite at ({},{})", x,y);
				let mut should_set_vf = false;
				for y in 0..height {
						let b = bus.ram_read_byte(self.i + y as u16);
						if bus.debug_draw_byte(b, x, y) {
								should_set_vf = true;
						}
				}
				if should_set_vf {
						
				} else {
						self.write_reg_vx(0xF, 0);
				}
				bus.present_screen();
		}

    pub fn run_instruction(&mut self, bus: &mut Bus) {
				let hi = bus.ram_read_byte(self.pc) as u16;
				let lo = bus.ram_read_byte(self.pc+1) as u16;
				let instruction:u16 = (hi <<8) | lo;
				println!("-----------------------------------------------------");
				println!("instruction read: {:#X} hi: {:#X} lo: {:#X}", instruction, hi, lo);
				let nnn = instruction  & 0x0FFF;
				let nn   = (instruction & 0x00FF) as u8;
				let n    = (instruction & 0x000F) as u8;
				let x   = ((instruction & 0x0F00) >> 8) as u8;
				let y   = ((instruction & 0x00F0) >> 4) as u8;
				println!("nnn = {:?} nn = {:?} n = {:?} x = {:?} y = {:?}", nnn, nn, n, x, y);
				println!("-----------------------------------------------------");

				if self.prev_pc == self.pc {
						panic!("Please increment PC!");
				}
				self.prev_pc = self.pc;
				
				match (instruction & 0xF000) >> 12 {
						0x0 => {
								match nn {
										0xE0 => {
												bus.clear_screen();
												self.pc += 2;
										},
										0xEE => {
												let addr = self.ret_stack.pop().unwrap();
												self.pc = addr;
										},
										_ => panic!("Unrecognized 00Instruction {:#X}:{:#X}", self.pc, instruction),
								}
						},
						0x1 => {
								// goto nnn
								self.pc = nnn;
						},
						0x2 => {
								// call subroutine
								self.ret_stack.push(self.pc + 2);
								self.pc = nnn;
						},
						0x3 => {
								let vx = self.read_reg_vx(x);
								if vx == nn {
										self.pc += 4;
								} else {
										self.pc += 2;
								}
						},
						0x6 => {
								//vx = nn								
								self.write_reg_vx(x, nn);
								self.pc += 2;
						},
						0x7 => {
								let vx = self.read_reg_vx(x);
								self.write_reg_vx(x, vx.wrapping_add(nn));
								self.pc += 2;
						},
						0x8 => {
								let vx = self.read_reg_vx(x);
								let vy = self.read_reg_vx(y);										
								
								match n {
										0 => {
												//Vx = Vy
												let vy = self.read_reg_vx(y);
												self.write_reg_vx(x, vy);
										},
										1 => {
												// Vx = Vx&Vy
												self.write_reg_vx(x, vx|vy);
														
										},
										2 => {
												// Vx = Vx&Vy
												self.write_reg_vx(x, vx&vy);
														
										},
										3 => {
 												// Vx = Vx^Vy
												self.write_reg_vx(x, vx^vy);
										},
										4 => {
 												// Vx = Vx+Vy
												let sum = vx as u16 + vy as u16;
												self.write_reg_vx(x, sum as u8);
												if sum > 0xFF {
														self.write_reg_vx(0xF, 1);
												}
										},
										5 => {
 												// Vx = Vx-Vy
												let diff: i8 = vx as i8 - vy as i8;
												self.write_reg_vx(x, diff as u8);												
												if diff < 0 {
														self.write_reg_vx(0xF, 1);
												}

										},
										6 => {
												let bit = vy & 0x1;
												self.write_reg_vx(y, vy >> 0x1);
												self.write_reg_vx(x, vy >> 0x1);
										},
										
										_ => panic!("Unrecognized 0x8XY* Instruction {:#X}:{:#X}", self.pc, instruction)
								}
								self.pc +=2;								
						},
						0xA => {
								self.i = nnn;
								self.pc += 2;
						},
						
						0xD => {
								//draw(Vx,Vy,N)
								self.debug_draw_sprite(bus, x, y, n);
								self.pc += 2;
						},
						0xE => {
								match nn {
										0xA1 => {
												// if key != Vx then skip the next instruction
												let key = self.read_reg_vx(x);
 												if !bus.key_pressed(key) {
														self.pc += 4;
												} else {
														self.pc += 2;
												}
										},
										0x9E => {
												// if key == Vx then skip the next instruction
												let key = self.read_reg_vx(x);
 												if bus.key_pressed(key) {
														self.pc += 4;
												} else {
														self.pc += 2;														
												}												
										}
										_ => panic!("Unrecognized Instruction {:#X}:{:#X}", self.pc, instruction),
								}	
						},
						
						0xF => {
								match nn {
										0x07 => {
												self.write_reg_vx(x, bus.get_delay_timer());
												self.pc += 2;												
										},
										0x15 => {
												bus.set_delay_timer(self.read_reg_vx(x));
												self.pc += 2;												
										},
										0x65 => {
												for index in 0..x+1 {
														let value = bus.ram_read_byte(self.i + index as u16);
														self.write_reg_vx(index, value);
												}
												self.pc += 2;												
										},
										//I += Vx;
										0x1E => {
												let vx = self.read_reg_vx(x);
												self.i += vx as u16;
												self.pc += 2;
										}
										_ => panic!("Unrecognized 0xF Instruction {:#X}:{:#X}", self.pc, instruction),
								}

						},
						_ => panic!("Unrecognized Instruction {:#X}:{:#X}", self.pc, instruction)
								
				}
		}
}

#[allow(unused_must_use)]
impl fmt::Debug for Cpu {
		fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
				writeln!(f, "pc: {:#X}", self.pc);
				writeln!(f, "vx: ");
				for item in self.vx.iter() {
						write!(f, "{:#X} ", *item);
				}
				writeln!(f, "\ni: {:#X}", self.i)
		}
}
