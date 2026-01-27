#[derive(Debug)]
pub struct Memory {
    rom: [u8; 0x2000],
    ram: [u8; 0x0400],
    vram: [u8; 0x1C00],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x2000],
            ram: [0; 0x0400],
            vram: [0; 0x1C00],
        }
    }

    pub fn init(&mut self, rom_data: [u8; 0x2000]) {
        self.rom = rom_data.clone();
        self.ram = [0; 0x0400];
        self.vram = [0; 0x1C00];
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
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

    pub fn read_word(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1fff => {
                let bl = (self.rom[addr as usize]) as u16;
                let bh = self.rom[(addr + 1) as usize] as u16;
                (bh << 8) | bl
            }
            0x2000..=0x23FF => {
                let n_addr = addr - 0x2000;
                let bl = (self.ram[n_addr as usize]) as u16;
                let bh = self.ram[(n_addr + 1) as usize] as u16;
                (bh << 8) | bl
            }
            0x2400..=0x3FFF => {
                let n_addr = addr - 0x2400;
                let bl = (self.vram[n_addr as usize]) as u16;
                let bh = self.vram[(n_addr + 1) as usize] as u16;
                (bh << 8) | bl
            }
            _ => panic!("out of memory"),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.rom[addr as usize] = value,
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
    pub fn write_word(&mut self, addr: u16, value: u16) {
        let lb = (value & 0x00FF) as u8;
        let hb = (value >> 8) as u8;
        match addr {
            0x0000..=0x1fff => {
                self.rom[addr as usize] = lb;
                self.rom[(addr + 1) as usize] = hb;
            }
            0x2000..=0x23FF => {
                let n_addr = addr - 0x2000;
                self.ram[n_addr as usize] = lb;
                self.ram[(n_addr + 1) as usize] = hb;
            }
            0x2400..=0x3FFF => {
                let n_addr = addr - 0x2400;
                self.vram[n_addr as usize] = lb;
                self.vram[(n_addr + 1) as usize] = hb;
            }
            _ => panic!("out of memory"),
        }
    }
}
