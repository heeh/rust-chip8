use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::ram::Ram;
use std::fmt;
use minifb::Window;

pub struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
    delay_timer: u8,
}

impl Bus {
    pub fn new() -> Bus {
	Bus {
	    ram: Ram::new(),
	    display: Display::new(),
	    keyboard: Keyboard::new(),
	    delay_timer: 0,
	    
	}
    }
    pub fn ram_read_byte(&self, address: u16) -> u8{
	self.ram.read_byte(address)
    }
    pub fn ram_write_byte(&mut self, address: u16, value: u8){
	self.ram.write_byte(address, value);
    }
    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool{
	self.display.debug_draw_byte(byte, x, y)
    }
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
	self.keyboard.set_key_pressed(key);
    }
    pub fn is_key_pressed(&self, key_code: u8) -> bool{
	self.keyboard.is_key_pressed(key_code)
    }
    pub fn get_key_pressed(&self) -> Option<u8> {
	self.keyboard.get_key_pressed()
    }
    
    pub fn present_screen(&self) {
	self.display.present();
    }
    pub fn clear_screen(&mut self) {
	self.display.clear();
    }
    pub fn delay_timer(&mut self, value:u8) {
	self.delay_timer = value;
    }
    pub fn tick(&mut self) {
	if self.delay_timer > 0 {
	    self.delay_timer -= 1;
	}
    }
    pub fn set_delay_timer(&mut self, value:u8) {
	self.delay_timer = value;
    }
    pub fn get_delay_timer(&self) -> u8 {
 	self.delay_timer
    }

    pub fn get_display_buffer(&self) ->&[u8] {
	self.display.get_display_buffer()
    }


}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "\nDelay timer: {:?}", self.delay_timer)
    }
}
