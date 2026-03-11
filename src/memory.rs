#[derive(Debug)]
pub struct Memory {
    rom: [u8; 0x2000],  // 8k
    ram: [u8; 0x0400],  // 1k
    vram: [u8; 0x1C00], // 7k
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x2000],
            ram: [0; 0x0400],
            vram: [0; 0x1C00],
        }
    }

    pub fn init_rom(&mut self, rom_data: Vec<u8>) {
        self.rom[..rom_data.len()].copy_from_slice(&rom_data);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        // Space Invaders hardware mirrors every 16KB (0x4000)
        // across the entire 64KB range.
        let addr = addr % 0x4000;
        match addr {
            0x0000..=0x1fff => self.rom[addr as usize],
            0x2000..=0x23FF => {
                let n_addr = addr - 0x2000;
                self.ram[n_addr as usize]
            }
            0x2400..=0x3FFF => {
                let n_addr = addr - 0x2400;
                self.vram[n_addr as usize]
            }

            a => panic!("out of memory: {:X} ", a),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        let addr = addr % 0x4000;
        match addr {
            0x0000..=0x1fff => {/* ROM is read-only */ },
            0x2000..=0x23FF => {
                let n_addr = addr - 0x2000;
                self.ram[n_addr as usize] = value
            }
            0x2400..=0x3FFF => {
                let n_addr = addr - 0x2400;
                self.vram[n_addr as usize] = value
            }
            _ => panic!("out of memory: {:X}", addr),
        }
    }


    pub fn read_word(&self, addr: u16) -> u16 {
        let l = self.read_byte(addr) as u16;
        let h = self.read_byte(addr.wrapping_add(1)) as u16;
        (h << 8) | l
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }

}
