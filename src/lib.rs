pub mod memory;
pub mod cpu;

use memory::Memory;
use cpu::Cpu;

use std::fs;

pub fn run(rom_file: &str) {
    let rom_data = fs::read(rom_file).unwrap();
    let mut memory = Memory::init(rom_data[0..8000].try_into().unwrap());

    let mut cpu = Cpu::new();
    loop {
        cpu.run(&mut memory);
    }


}