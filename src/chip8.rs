use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct Chip8 {
    cpu: Cpu,
		bus: Bus,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
						bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus.ram_write_byte(0x200 + (i as u16), data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
				println!("Cpu state: {:?}", self.cpu);
    }
}
