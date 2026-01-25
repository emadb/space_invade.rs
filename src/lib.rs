pub mod cpu;
pub mod memory;
pub mod stack;

use cpu::Cpu;
use memory::Memory;

use std::fs;

pub fn run(rom_file: &str) {
    let rom_data = fs::read(rom_file).unwrap();
    let mut memory = Memory::init(rom_data[0..8000].try_into().unwrap());

    let mut cpu = Cpu::new();
    loop {
        cpu.run(&mut memory);
        // let mut str = String::new();
        // let _ = std::io::stdin().read_line(&mut str);
    }
}
