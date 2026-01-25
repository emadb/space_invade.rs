use crate::memory::Memory;
use std::fmt;

const ZERO: u8 = 0b00000000;
const FLAG_SIGN: u8 = 0b10000000;
const FLAG_ZERO: u8 = 0b01000000;
const FLAG_AUX_CARRY: u8 = 0b00010000;
const FLAG_PARITY: u8 = 0b00000100;
const FLAG_CARRY: u8 = 0b00000001;

struct Cycles(u8);

struct Flags(u8);

impl Flags {
    fn is_zero(&self) -> bool {
        self.0 & FLAG_ZERO != 0
    }

    fn get_carry(&self, v1: u8, v2: u8) -> bool {
        v1 < v2
    }

    fn get_half_carry(&self, v1: u8, v2: u8) -> bool {
        (v1 & 0x0F) < (v2 & 0x0F)
    }

    // S - Sign Flag
    // Z - Zero Flag
    // 0 - Not used, always zero
    // A - also called AC, Auxiliary Carry Flag
    // 0 - Not used, always zero
    // P - Parity Flag
    // 1 - Not used, always one
    // C - Carry Flag
    pub fn set(&mut self, value: u8, half_carry: Option<bool>, carry: Option<bool>) {
        let s = (value & 0x80) != 0;
        let z = value == 0;
        let p = value.count_ones() % 2 == 0;

        let bs = if s { FLAG_SIGN } else { ZERO };
        let bz = if z { FLAG_ZERO } else { ZERO };
        let b0 = ZERO;
        let b1 = ZERO;
        let bp = if p { FLAG_PARITY } else { ZERO };
        let b2 = 0b00000010;

        let ba = match half_carry {
            Some(h) => {
                if h {
                    FLAG_AUX_CARRY
                } else {
                    ZERO
                }
            }
            None => self.0 & FLAG_AUX_CARRY,
        };

        let bc = match carry {
            Some(c) => {
                if c {
                    FLAG_CARRY
                } else {
                    ZERO
                }
            }
            None => self.0 & FLAG_CARRY,
        };

        self.0 = bs | bz | b0 | ba | b1 | bp | b2 | bc;
    }
}

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
    // S - Sign Flag
    // Z - Zero Flag
    // 0 - Not used, always zero
    // A - also called AC, Auxiliary Carry Flag
    // 0 - Not used, always zero
    // P - Parity Flag
    // 1 - Not used, always one
    // C - Carry Flag
    flags: Flags,
    cycles: u64,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cpu Registers: af={:X}{:X} bc={:X}{:X} de={:X}{:X} hl={:X}{:X} pc: {:X} sp: {:X} flags: {:b} cycles: {}",
            self.a,
            self.flags.0,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.pc,
            self.sp,
            self.flags.0,
            self.cycles
        )
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xF000,
            pc: 0,
            flags: Flags(0),
            cycles: 0,
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.pc);
        self.pc += 1;
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let l_addr = self.fetch_byte(memory);
        let h_addr = self.fetch_byte(memory);

        ((h_addr as u16) << 8) | (l_addr as u16)
    }

    pub fn run(&mut self, memory: &mut Memory) {
        let opcode = self.fetch_byte(memory);
        println!("OP: {:#4X} - {:?}", opcode, self);
        let cy = self.execute_instruction(opcode, memory);
        self.cycles += cy.0 as u64;
    }

    fn execute_instruction(&mut self, opcode: u8, memory: &mut Memory) -> Cycles {
        let cycles = match opcode {
            0x00 => self.no_op(memory),
            0x01 => self.lxi_b(memory),
            0x02 => self.stax_b(memory),
            0x03 => self.inx_b(memory),
            0x04 => self.inr_b(memory),
            0x05 => self.dcr_b(memory),
            0x06 => self.mvi_b(memory),
            0x11 => self.lxi_d(memory),
            0x13 => self.inx_d(memory),
            0x1A => self.ldax_d(memory),
            0x1B => self.dcx_d(memory),
            0x20 => self.no_op(memory),
            0x21 => self.lxi_h(memory),
            0x23 => self.inx_h(memory),
            0x31 => self.lxi_sp(memory),
            0x32 => self.sta(memory),
            0x36 => self.mvi_m(memory),
            0x77 => self.mov_ma(memory),
            0x7C => self.mov_ah(memory),
            0xC2 => self.jnz(memory),
            0xC3 => self.jmp(memory),
            0xC9 => self.ret(memory),
            0xCD => self.call(memory),
            0xFE => self.cpi(memory),

            op => panic!("Unknown opcode: {:#2X}", op),
        };
        cycles
    }
    fn no_op(&self, _mem: &Memory) -> Cycles {
        Cycles(4)
    }

    fn lxi_b(&mut self, mem: &Memory) -> Cycles {
        let w = self.fetch_word(mem);
        set_16(&mut self.b, &mut self.c, w);
        Cycles(10)
    }

    fn lxi_d(&mut self, mem: &Memory) -> Cycles {
        let w = self.fetch_word(mem);
        set_16(&mut self.d, &mut self.e, w);
        Cycles(10)
    }

    fn lxi_h(&mut self, mem: &Memory) -> Cycles {
        let w = self.fetch_word(mem);
        set_16(&mut self.h, &mut self.l, w);
        Cycles(10)
    }

    fn stax_b(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.b, self.c);
        mem.write_byte(addr, self.a);
        Cycles(7)
    }

    fn inx_b(&mut self, _mem: &mut Memory) -> Cycles {
        let value = get_16(self.b, self.c);
        set_16(&mut self.b, &mut self.c, value.wrapping_add(1));
        Cycles(5)
    }

    fn inx_d(&mut self, _mem: &mut Memory) -> Cycles {
        let value = get_16(self.d, self.e);
        set_16(&mut self.d, &mut self.e, value.wrapping_add(1));
        Cycles(5)
    }

    fn inx_h(&mut self, _mem: &mut Memory) -> Cycles {
        let value = get_16(self.h, self.l);
        set_16(&mut self.h, &mut self.l, value.wrapping_add(1));
        Cycles(5)
    }

    fn dcx_d(&mut self, _mem: &mut Memory) -> Cycles {
        let value = get_16(self.d, self.e);
        set_16(&mut self.d, &mut self.e, value.wrapping_sub(1));
        Cycles(5)
    }

    fn inr_b(&mut self, _mem: &mut Memory) -> Cycles {
        let hc = (self.b & 0x0F) == 0x0F; // Half-carry on overflow from bit 3
        let new_value = self.b.wrapping_add(1);
        self.flags.set(new_value, Some(hc), None);
        self.b = new_value;
        Cycles(5)
    }

    fn dcr_b(&mut self, _mem: &mut Memory) -> Cycles {
        let hc = (self.b & 0x0F) == 0x00; // Half-carry on borrow from bit 4
        let new_value = self.b.wrapping_sub(1);
        self.flags.set(new_value, Some(hc), None);
        self.b = new_value;
        Cycles(5)
    }

    fn mvi_b(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.b = value;
        Cycles(7)
    }

    fn lxi_sp(&mut self, mem: &Memory) -> Cycles {
        let value = self.fetch_word(mem);

        self.sp = value;
        Cycles(10)
    }

    fn ldax_d(&mut self, mem: &Memory) -> Cycles {
        let addr = get_16(self.d, self.e);
        self.a = mem.read_byte(addr);
        Cycles(7)
    }

    fn sta(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        mem.write_byte(addr, self.a);
        Cycles(13)
    }

    fn mov_ma(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        mem.write_byte(addr, self.a);
        Cycles(7)
    }

    fn mov_ah(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.h;
        Cycles(5)
    }

    fn mvi_m(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = self.fetch_byte(mem);
        mem.write_byte(addr, value);
        Cycles(10)
    }

    fn jnz(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        if !self.flags.is_zero() {
            self.pc = addr;
        }
        Cycles(10)
    }

    fn jmp(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_word(mem);
        self.pc = value;
        Cycles(10)
    }

    fn ret(&mut self, mem: &mut Memory) -> Cycles {
        let addr = mem.read_word(self.sp); // Fixed: read before incrementing SP
        self.sp += 2;
        self.pc = addr;
        Cycles(10)
    }

    fn call(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);

        let h = (self.pc >> 8) as u8;
        let l = (self.pc & 0xFF) as u8;

        mem.write_byte(self.sp - 1, h);
        mem.write_byte(self.sp - 2, l);
        self.sp -= 2;
        self.pc = addr;
        Cycles(17)
    }

    fn cpi(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        println!("CPI: A={:02X} compare with {:02X}", self.a, value); // Debug output
        let res = self.a.wrapping_sub(value);
        let hc = self.flags.get_half_carry(self.a, value);
        let c = self.flags.get_carry(self.a, value);

        self.flags.set(res, Some(hc), Some(c));
        Cycles(7)
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
