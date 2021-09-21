extern crate minifb;

use minifb::{Key, Window, WindowOptions, KeyRepeat};

use crate::chip8::Chip8;
use std::fs::File;
use std::io::Read;


mod chip8;
mod cpu;
mod ram;
mod keyboard;
mod display;
mod bus;

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8>{
    match key {
	Some(Key::Key1) => Some(0x1),
	Some(Key::Key2) => Some(0x2),
	Some(Key::Key3) => Some(0x3),
	Some(Key::Key4) => Some(0xC),
	Some(Key::Apostrophe) => Some(0x4),
	Some(Key::Comma) => Some(0x5),
	Some(Key::Period) => Some(0x6),
	Some(Key::P) => Some(0xD),		
	Some(Key::A) => Some(0x7),
	Some(Key::O) => Some(0x8),
	Some(Key::E) => Some(0x9),
 	Some(Key::U) => Some(0xE),		
	Some(Key::Semicolon) => Some(0xA),
	Some(Key::Q) => Some(0x0),
	Some(Key::J) => Some(0xB),
	Some(Key::K) => Some(0xF),
	_ => None
    }    
}

#[allow(unused_must_use)]
fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);


    let width = 640;
    let height = 320;
    
    let mut buffer: Vec<u32> = vec![0; width * height];

    // for i in buffer.iter_mut() {
    // 		*i = 0xffff0000;
    // }
    
    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
	.unwrap_or_else(|e| {
            panic!("{}", e);
	});

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
	let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
	let key = match keys_pressed {
	    Some(keys) => {
		if !keys.is_empty() {
		    Some(keys[0])
		} else {
		    None
		}
	    }
	    None => None
	};
	let chip8_key = get_chip8_keycode_for(key);
	chip8.set_key_pressed(chip8_key);

	
	chip8.run_instruction();
	let chip8_buffer = chip8.get_display_buffer();

	for y in 0..height {
	    for x in 0..width {
		let index = y/10 * width/10 + x/10;
		let pixel = chip8_buffer[index];
		let color_pixel = match pixel {
		    0 => 0x0,
		    1 => 0xffffff,
		    _ => unreachable!()
		};
		buffer[y * width + x] = color_pixel;
	    }
	}

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}
