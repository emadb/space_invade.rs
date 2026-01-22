use crate::memory::Memory;

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    flags: u8
}

impl Cpu {
    pub fn new() -> Self {
        Cpu{
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            flags: 0
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
          let byte = memory.read_byte(self.pc);
          self.pc += 1;
          byte
      }


    pub fn run(&mut self, memory: &mut Memory) {
        let opcode = self.fetch_byte(memory);
        println!("PC: {:X}  OP: {:X}", self.pc, opcode);
        self.execute_instruction(opcode, memory);
    }

    fn execute_instruction(&mut self, opcode: u8, memory: &mut Memory){
        let _ = match opcode {
            0x00 => self.no_op(memory),
            0x01 => self.lxi_b(memory),
            0x02 => self.stax_b(memory),
            _ => panic!("Unknown opcode")
        };
    }
    fn no_op(&self, _mem: &Memory) {}

    fn lxi_b(&mut self, mem: &Memory) {
        let w = mem.read_word(self.pc);
        set_16(&mut self.b, &mut self.c, w)
    }

    fn stax_b(&mut self, mem: &mut Memory) {
        let addr = get_16(self.b, self.c);
        mem.write_byte(addr, self.a);
    }
}

pub fn get_16(h: u8, l: u8) -> u16 {
    (h as u16) << 8 | l as u16
}

fn set_16(h: &mut u8, l: &mut u8, value: u16) {
    *h = ((value & 0xFF00) >> 8) as u8;
    *l = (value & 0xFF) as u8;
}

//     pub fn fetch_byte(&mut self, memory: &impl Memory) -> u8 {
//         let byte = memory.read_byte(self.pc);
//         self.pc += 1;
//         byte
//     }

//     pub fn fetch_word(&mut self, memory: &impl Memory) -> u16 {
//         let low = self.fetch_byte(memory) as u16;
//         let high = self.fetch_byte(memory) as u16;
//         (high << 8) | low
//     }