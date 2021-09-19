use chip8::Chip8;
use std::fs::File;
use std::io::Read;

mod chip8;
mod ram;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    //print!("Data: {:?}", data);
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
}
