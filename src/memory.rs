#[derive(Debug)]
pub struct Memory {
    rom: [u8;8000],
    ram: [u8;8000]
}

impl Memory {
    pub fn init(rom_data: [u8;8000]) -> Self {
        Memory {
            rom: rom_data.clone(),
            ram: [0;8000]
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.rom[addr as usize],
            0x2000..0x3fff => self.ram[addr as usize],
            _ => panic!("out of memory")
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        match addr {
            0x0000..=0x1fff => {
                let bh = (self.rom[addr as usize] << 8) as u16;
                let bl = self.rom[(addr + 1) as usize] as u16;
                (bh << 8) | bl
            },
            0x2000..0x3fff => {
                let bh = (self.ram[addr as usize] << 8) as u16;
                let bl = self.ram[(addr + 1) as usize] as u16;
                (bh << 8) | bl
            },
            _ => panic!("out of memory")
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.rom[addr as usize] = value,
            0x2000..0x3fff => self.ram[addr as usize] = value,
            _ => panic!("out of memory")
        }
    }
}