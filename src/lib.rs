pub mod cpu;
pub mod memory;
pub mod stack;
pub mod bus;

use cpu::Cpu;
use memory::Memory;
use bus::Bus;

use std::fs;

pub fn run(rom_file: &str) {
    let rom_data = fs::read(rom_file).unwrap();
    let mut memory = Memory::init(rom_data[0..0x2000].try_into().unwrap());
    let mut bus = Bus{};

    let mut cpu = Cpu::new();
    loop {
        cpu.run(&mut memory, &mut bus);
        // let mut str = String::new();
        // let _ = std::io::stdin().read_line(&mut str);
    }
}
