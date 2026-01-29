use crate::{bus::Bus, cpu::flags::Flags, memory::Memory};
use std::fmt;

#[derive(PartialEq)]
enum Interrupt {
    Enabled,
    Disabled
}

struct Cycles(u8);

pub struct Cpu {
    ei: Interrupt,
    pending_int: Option<u16>,
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
            "af={:02X}{:02X} bc={:02X}{:02X} de={:02X}{:02X} hl={:02X}{:02X} pc:{:04X} sp:{:04X} flags: {:08b}",
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
            self.flags.0
        )
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            ei: Interrupt::Disabled,
            pending_int: None,
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

    pub fn run_step(&mut self, memory: &mut Memory, bus: &mut Bus) {
        let opcode = self.fetch_byte(memory);
        println!("OP: {:#4X} - {:?}", opcode, self);
        let cy = self.execute_instruction(opcode, memory, bus);
        self.cycles += cy.0 as u64;
    }


    pub fn send_interrupt(&mut self, int: u16) {
        if self.ei == Interrupt::Enabled {
            self.pending_int = Some(int)
        }
    }

    fn process_interrupt(&mut self, mem: &mut Memory) {
        if self.ei == Interrupt::Enabled && let Some(addr) = self.pending_int {

            // TOOD: create a fn (push the sp)
            let h = (self.pc >> 8) as u8;
            let l = (self.pc & 0xFF) as u8;
            mem.write_byte(self.sp - 1, h);
            mem.write_byte(self.sp - 2, l);

            self.sp -= 2;
            self.pc = addr & 0x0038;
            self.pending_int = None;
        }
    }

    fn execute_instruction(&mut self, opcode: u8, memory: &mut Memory, bus: &mut Bus) -> Cycles {

        self.process_interrupt(memory);

        let cycles = match opcode {
            0x00 => self.no_op(memory),
            0x01 => self.lxi_b(memory),
            0x02 => self.stax_b(memory),
            0x03 => self.inx_b(memory),
            0x04 => self.inr_b(memory),
            0x05 => self.dcr_b(memory),
            0x06 => self.mvi_b(memory),
            0x07 => self.rlc(memory),
            0x09 => self.dad_b(memory),
            0x0A => self.ldax_b(memory),
            //0x0B => self.(memory),
            //0x0C => self.(memory),
            0x0D => self.dcr_c(memory),
            0x0E => self.mvi_c(memory),
            0x0F => self.rrc(memory),
            //0x10 => self.(memory),
            0x11 => self.lxi_d(memory),
            //0x12 => self.(memory),
            0x13 => self.inx_d(memory),
            // 0x14 => self.(memory),
            // 0x15 => self.(memory),
            0x16 => self.mvi_d(memory),
            // 0x17 => self.(memory),
            // 0x18 => self.(memory),
            0x19 => self.dad_d(memory),
            0x1A => self.ldax_d(memory),
            0x1B => self.dcx_d(memory),
            0x1C => self.inr_e(memory),
            // 0x1D => self.(memory),
            // 0x1E => self.(memory),
            // 0x1F => self.(memory),
            0x20 => self.no_op(memory),
            0x21 => self.lxi_h(memory),
            // 0x22 => self.(memory),
            0x23 => self.inx_h(memory),
            // 0x24 => self.(memory),
            // 0x25 => self.(memory),
            0x26 => self.mvi_h(memory),
            // 0x27 => self.(memory),
            // 0x28 => self.(memory),
            0x29 => self.dad_h(memory),
            // 0x2A => self.(memory),
            // 0x2B => self.(memory),
            // 0x2C => self.(memory),
            // 0x2D => self.(memory),
            0x2E => self.mvi_l(memory),
            // 0x2F => self.(memory),
            // 0x30 => self.(memory),
            0x31 => self.lxi_sp(memory),
            0x32 => self.sta(memory),
            // 0x33 => self.(memory),
            // 0x34 => self.(memory),
            0x35 => self.dcr_m(memory),
            0x36 => self.mvi_m(memory),
            0x37 => self.stc(memory),
            // 0x38 => self.(memory),
            // 0x39 => self.(memory),
            0x3A => self.lda(memory),
            // 0x3B => self.(memory),
            // 0x3C => self.(memory),
            0x3D => self.dcr_a(memory),
            0x3E => self.mvi_a(memory),
            // 0x3F => self.(memory),
            0x40 => self.mov_bb(memory),
            0x41 => self.mov_bc(memory),
            0x42 => self.mov_bd(memory),
            0x43 => self.mov_be(memory),
            0x44 => self.mov_bh(memory),
            0x45 => self.mov_bl(memory),
            0x46 => self.mov_bm(memory),
            0x47 => self.mov_ba(memory),
            0x48 => self.mov_cb(memory),
            0x49 => self.mov_cc(memory),
            0x4A => self.mov_cd(memory),
            0x4B => self.mov_ce(memory),
            0x4C => self.mov_ch(memory),
            0x4D => self.mov_cl(memory),
            0x4E => self.mov_cm(memory),
            0x4F => self.mov_ca(memory),
            0x50 => self.mov_db(memory),
            0x51 => self.mov_dc(memory),
            0x52 => self.mov_dd(memory),
            0x53 => self.mov_de(memory),
            0x54 => self.mov_dh(memory),
            0x55 => self.mov_dl(memory),
            0x56 => self.mov_dm(memory),
            0x57 => self.mov_da(memory),
            0x58 => self.mov_eb(memory),
            0x59 => self.mov_ec(memory),
            0x5A => self.mov_ed(memory),
            0x5B => self.mov_ee(memory),
            0x5C => self.mov_eh(memory),
            0x5D => self.mov_el(memory),
            0x5E => self.mov_em(memory),
            0x5F => self.mov_ea(memory),
            0x60 => self.mov_hb(memory),
            0x61 => self.mov_hc(memory),
            0x62 => self.mov_hd(memory),
            0x63 => self.mov_he(memory),
            0x64 => self.mov_hh(memory),
            0x65 => self.mov_hl(memory),
            0x66 => self.mov_hm(memory),
            0x67 => self.mov_ha(memory),
            0x68 => self.mov_lb(memory),
            0x69 => self.mov_lc(memory),
            0x6A => self.mov_ld(memory),
            0x6B => self.mov_le(memory),
            0x6C => self.mov_lh(memory),
            0x6D => self.mov_ll(memory),
            0x6E => self.mov_lm(memory),
            0x6F => self.mov_la(memory),

            // 0x70 => self.(memory),
            // 0x71 => self.(memory),
            // 0x72 => self.(memory),
            // 0x73 => self.(memory),
            // 0x74 => self.(memory),
            // 0x75 => self.(memory),
            // 0x76 => self.(memory),
            0x77 => self.mov_ma(memory),
            0x78 => self.mov_ab(memory),
            0x79 => self.mov_ac(memory),
            0x7A => self.mov_ad(memory),
            0x7B => self.mov_ae(memory),
            0x7C => self.mov_ah(memory),
            0x7D => self.mov_al(memory),
            0x7E => self.mov_am(memory),
            // 0x7F => self.(memory),
            // 0x80 => self.(memory),
            // 0x81 => self.(memory),
            // 0x82 => self.(memory),
            // 0x83 => self.(memory),
            // 0x84 => self.(memory),
            // 0x85 => self.(memory),
            // 0x86 => self.(memory),
            // 0x87 => self.(memory),
            // 0x88 => self.(memory),
            // 0x89 => self.(memory),
            // 0x8A => self.(memory),
            // 0x8B => self.(memory),
            // 0x8C => self.(memory),
            // 0x8D => self.(memory),
            // 0x8E => self.(memory),
            // 0x8F => self.(memory),
            // 0x90 => self.(memory),
            // 0x91 => self.(memory),
            // 0x92 => self.(memory),
            // 0x93 => self.(memory),
            // 0x94 => self.(memory),
            // 0x95 => self.(memory),
            // 0x96 => self.(memory),
            // 0x97 => self.(memory),
            // 0x98 => self.(memory),
            // 0x99 => self.(memory),
            // 0x9A => self.(memory),
            // 0x9B => self.(memory),
            // 0x9C => self.(memory),
            // 0x9D => self.(memory),
            0x9E => self.sbb_m(memory),
            // 0x9F => self.(memory),
            // 0xA0 => self.(memory),
            // 0xA1 => self.(memory),
            // 0xA2 => self.(memory),
            // 0xA3 => self.(memory),
            // 0xA4 => self.(memory),
            // 0xA5 => self.(memory),
            // 0xA6 => self.(memory),
            0xA7 => self.ana_a(memory),
            // 0xA8 => self.(memory),
            // 0xA9 => self.(memory),
            // 0xAA => self.(memory),
            // 0xAB => self.(memory),
            // 0xAC => self.(memory),
            // 0xAD => self.(memory),
            // 0xAE => self.(memory),
            0xAF => self.xra(memory),
            0xB0 => self.ora_b(memory),
            // 0xB1 => self.(memory),
            // 0xB2 => self.(memory),
            // 0xB3 => self.(memory),
            // 0xB4 => self.(memory),
            // 0xB5 => self.(memory),
            0xB6 => self.add_m(memory),
            // 0xB7 => self.(memory),
            // 0xB8 => self.(memory),
            // 0xB9 => self.(memory),
            // 0xBA => self.(memory),
            // 0xBB => self.(memory),
            // 0xBC => self.(memory),
            // 0xBD => self.(memory),
            // 0xBE => self.(memory),
            // 0xBF => self.(memory),
            0xC0 => self.rnz(memory),
            0xC1 => self.pop_b(memory),
            0xC2 => self.jnz(memory),
            0xC3 => self.jmp(memory),
            0xC4 => self.cnz(memory),
            0xC5 => self.push_b(memory),
            0xC6 => self.adi(memory),
            // 0xC7 => self.(memory),
            0xC8 => self.rz(memory),
            0xC9 => self.ret(memory),
            0xCA => self.jz(memory),
            // 0xCB => self.(memory),
            // 0xCC => self.(memory),
            0xCD => self.call(memory),
            // 0xCE => self.(memory),
            // 0xCF => self.(memory),
            0xD0 => self.rnc(memory),
            0xD1 => self.pop_d(memory),
            0xD2 => self.jnc(memory),
            0xD3 => self.out(memory, bus),
            // 0xD4 => self.(memory),
            0xD5 => self.push_d(memory),
            0xD6 => self.sui(memory),
            // 0xD7 => self.(memory),
            0xD8 => self.rc(memory),
            // 0xD9 => self.(memory),
            0xDA => self.jc(memory),
            0xDB => self.inp(memory, bus),
            // 0xDC => self.(memory),
            // 0xDD => self.(memory),
            // 0xDE => self.(memory),
            // 0xDF => self.(memory),
            0xE0 => self.rpo(memory),
            0xE1 => self.pop_h(memory),
            // 0xE2 => self.(memory),
            // 0xE3 => self.(memory),
            // 0xE4 => self.(memory),
            0xE5 => self.push_h(memory),
            0xE6 => self.ani(memory),
            // 0xE7 => self.(memory),
            // 0xE8 => self.(memory),
            // 0xE9 => self.(memory),
            // 0xE0 => self.(memory),
            // 0xEA => self.(memory),
            0xEB => self.xcgh(memory),
            // 0xEC => self.(memory),
            // 0xED => self.(memory),
            // 0xEE => self.(memory),
            // 0xEF => self.(memory),
            0xF0 => self.rp(memory),
            0xF1 => self.pop_psw(memory),
            // 0xF2 => self.(memory),
            // 0xF3 => self.(memory),
            // 0xF4 => self.(memory),
            0xF5 => self.push_psw(memory),
            0xF6 => self.ori(memory),
            // 0xF7 => self.(memory),
            // 0xF8 => self.(memory),
            // 0xF9 => self.(memory),
            // 0xFA => self.(memory),
            0xFB => self.ei(memory),
            // 0xFC => self.(memory),
            // 0xFD => self.(memory),
            0xFE => self.cpi(memory),
            // 0xFF => self.(memory),

            op => panic!("Unknown opcode: {:#2X}", op),
        };
        cycles
    }
    fn no_op(&self, _mem: &Memory) -> Cycles {
        Cycles(4)
    }

    fn ani(&mut self, mem: &Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.a = self.a & value;
        let hc = 0b00001000 & self.a != 0;
        self.flags.set(self.a, Some(hc), Some(false));
        Cycles(7)
    }

    fn ori(&mut self, mem: &Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.a = self.a | value;
        self.flags.set(self.a, Some(false), Some(false));
        Cycles(7)
    }

    fn ana_a(&mut self, _mem: &Memory) -> Cycles {
        // self.a = self.a & self.a;
        let hc = 0b00001000 & self.a != 0;
        self.flags.set(self.a, Some(hc), Some(false));
        Cycles(4)
    }

    fn ora_b(&mut self, _mem: &Memory) -> Cycles {
        self.a = self.a | self.b;
        let hc = 0b00001000 & self.a != 0;
        self.flags.set(self.a, Some(hc), Some(false));
        Cycles(4)
    }

    fn adi(&mut self, mem: &Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        let (res, carry) = self.a.overflowing_add(value);
        let hc = (self.a & 0x0F) + (value & 0x0F) > 0x0F;
        self.flags.set(res, Some(hc), Some(carry));
        self.a = res;
        Cycles(7)
    }

    fn sui(&mut self, mem: &Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        let (res, carry) = self.a.overflowing_sub(value);
        let hc = (value & 0x0F) > (self.a & 0x0F);
        self.flags.set(res, Some(hc), Some(carry));
        self.a = res;
        Cycles(7)
    }

    fn ei(&mut self, _mem: &Memory) -> Cycles {
        self.ei = Interrupt::Enabled;
        Cycles(4)
    }

    fn out(&mut self, mem: &Memory, bus: &mut Bus) -> Cycles {
        let port = self.fetch_byte(mem);
        bus.write_port(port, self.a);
        Cycles(10)
    }

    fn inp(&mut self, mem: &Memory, bus: &mut Bus) -> Cycles {
        let port = self.fetch_byte(mem);
        self.a = bus.read_port(port);
        Cycles(10)
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
        let new_value = self.b.wrapping_add(1);
        let hc = (new_value & 0x0F) == 0;
        self.flags.set(new_value, Some(hc), None);
        self.b = new_value;
        Cycles(5)
    }

    fn inr_e(&mut self, _mem: &mut Memory) -> Cycles {
        let new_value = self.e.wrapping_add(1);
        let hc = (new_value & 0x0F) == 0;
        self.flags.set(new_value, Some(hc), None);
        self.e = new_value;
        Cycles(5)
    }

    fn dcr_a(&mut self, _mem: &mut Memory) -> Cycles {
        let new_value = self.a.wrapping_sub(1);
        let hc = (new_value & 0x0F) != 0x0F;
        self.flags.set(new_value, Some(hc), None);
        self.a = new_value;
        Cycles(5)
    }

    fn dcr_b(&mut self, _mem: &mut Memory) -> Cycles {
        let new_value = self.b.wrapping_sub(1);
        let hc = (new_value & 0x0F) != 0x0F;
        self.flags.set(new_value, Some(hc), None);
        self.b = new_value;
        Cycles(5)
    }

    fn dcr_c(&mut self, _mem: &mut Memory) -> Cycles {
        let new_value = self.c.wrapping_sub(1);
        let hc = (new_value & 0x0F) != 0x0F;
        self.flags.set(new_value, Some(hc), None);
        self.c = new_value;
        Cycles(5)
    }

    fn dcr_m(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        let new_value = value.wrapping_sub(1);
        let hc = (new_value & 0x0F) != 0x0F;
        self.flags.set(new_value, Some(hc), None);
        mem.write_byte(addr, new_value);
        Cycles(5)
    }

    fn dad_h(&mut self, _mem: &mut Memory) -> Cycles {
        let value = get_16(self.h, self.l);
        let (res, carry) = value.overflowing_add(value);
        set_16(&mut self.h, &mut self.l, res);
        self.flags.set_carry(carry);
        Cycles(10)
    }

    fn dad_b(&mut self, _mem: &mut Memory) -> Cycles {
        let hl = get_16(self.h, self.l);
        let bc = get_16(self.b, self.c);
        let (res, carry) = hl.overflowing_add(bc);
        set_16(&mut self.h, &mut self.l, res);
        self.flags.set_carry(carry);
        Cycles(10)
    }

    fn dad_d(&mut self, _mem: &mut Memory) -> Cycles {
        let hl = get_16(self.h, self.l);
        let de = get_16(self.d, self.e);
        let (res, carry) = hl.overflowing_add(de);
        set_16(&mut self.h, &mut self.l, res);
        self.flags.set_carry(carry);
        Cycles(10)
    }

    fn mvi_b(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.b = value;
        Cycles(7)
    }

    fn mvi_a(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.a = value;
        Cycles(7)
    }

    fn mvi_c(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.c = value;
        Cycles(7)
    }

    fn mvi_d(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.d = value;
        Cycles(7)
    }

    fn mvi_h(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.h = value;
        Cycles(7)
    }

    fn mvi_l(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_byte(mem);
        self.l = value;
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

    fn ldax_b(&mut self, mem: &Memory) -> Cycles {
        let addr = get_16(self.b, self.c);
        self.a = mem.read_byte(addr);
        Cycles(7)
    }

    fn lda(&mut self, mem: &Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        self.a = mem.read_byte(addr);
        Cycles(13)
    }

    fn sta(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        mem.write_byte(addr, self.a);
        Cycles(13)
    }

    fn stc(&mut self, _mem: &mut Memory) -> Cycles {
        self.flags.set_carry(true);
        Cycles(4)
    }


    fn mov_ma(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        mem.write_byte(addr, self.a);
        Cycles(7)
    }

    fn mov_am(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.a = value;
        Cycles(7)
    }

    fn mov_bm(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.b = value;
        Cycles(7)
    }

    fn mov_cm(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.c = value;
        Cycles(7)
    }


    fn mov_dm(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.d = value;
        Cycles(7)
    }

    fn mov_em(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.e = value;
        Cycles(7)
    }

    fn mov_hm(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.h = value;
        Cycles(7)
    }

    fn mov_lm(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);
        self.l = value;
        Cycles(7)
    }

    fn mov_ab(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.b;
        Cycles(5)
    }

    fn mov_ac(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.c;
        Cycles(5)
    }


    fn mov_ad(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.d;
        Cycles(5)
    }

    fn mov_ea(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.a;
        Cycles(5)
    }

    fn mov_ba(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.l;
        Cycles(5)
    }

    fn mov_bb(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.b;
        Cycles(5)
    }

    fn mov_bc(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.c;
        Cycles(5)
    }

    fn mov_bd(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.d;
        Cycles(5)
    }

    fn mov_be(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.e;
        Cycles(5)
    }

    fn mov_bh(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.h;
        Cycles(5)
    }

    fn mov_bl(&mut self, _mem: &mut Memory) -> Cycles {
        self.b = self.l;
        Cycles(5)
    }

    fn mov_ca(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.a;
        Cycles(5)
    }

    fn mov_cb(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.b;
        Cycles(5)
    }

    fn mov_cc(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.c;
        Cycles(5)
    }

    fn mov_cd(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.d;
        Cycles(5)
    }

    fn mov_ce(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.e;
        Cycles(5)
    }

    fn mov_ch(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.h;
        Cycles(5)
    }

    fn mov_cl(&mut self, _mem: &mut Memory) -> Cycles {
        self.c = self.l;
        Cycles(5)
    }

    fn mov_da(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.a;
        Cycles(5)
    }

    fn mov_db(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.b;
        Cycles(5)
    }

    fn mov_dc(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.c;
        Cycles(5)
    }

    fn mov_dd(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.d;
        Cycles(5)
    }

    fn mov_de(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.e;
        Cycles(5)
    }

    fn mov_dh(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.h;
        Cycles(5)
    }

    fn mov_dl(&mut self, _mem: &mut Memory) -> Cycles {
        self.d = self.l;
        Cycles(5)
    }

    fn mov_eb(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.b;
        Cycles(5)
    }

    fn mov_ec(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.c;
        Cycles(5)
    }

    fn mov_ed(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.d;
        Cycles(5)
    }

    fn mov_ee(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.e;
        Cycles(5)
    }

    fn mov_eh(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.h;
        Cycles(5)
    }

    fn mov_el(&mut self, _mem: &mut Memory) -> Cycles {
        self.e = self.l;
        Cycles(5)
    }

    fn mov_hb(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.b;
        Cycles(5)
    }

    fn mov_hc(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.c;
        Cycles(5)
    }

    fn mov_hd(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.d;
        Cycles(5)
    }

    fn mov_he(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.e;
        Cycles(5)
    }

    fn mov_hh(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.h;
        Cycles(5)
    }

    fn mov_hl(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.l;
        Cycles(5)
    }

    fn mov_lb(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.b;
        Cycles(5)
    }

    fn mov_lc(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.c;
        Cycles(5)
    }

    fn mov_ld(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.d;
        Cycles(5)
    }

    fn mov_le(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.e;
        Cycles(5)
    }

    fn mov_lh(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.h;
        Cycles(5)
    }

    fn mov_ll(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.l;
        Cycles(5)
    }

    fn mov_la(&mut self, _mem: &mut Memory) -> Cycles {
        self.l = self.a;
        Cycles(5)
    }

    fn mov_ae(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.e;
        Cycles(5)
    }

    fn mov_ah(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.h;
        Cycles(5)
    }

    fn mov_al(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.l;
        Cycles(5)
    }

    fn mov_ha(&mut self, _mem: &mut Memory) -> Cycles {
        self.h = self.a;
        Cycles(5)
    }

    fn xcgh(&mut self, _mem: &mut Memory) -> Cycles {
        let temp_h = self.h;
        let temp_l = self.l;

        self.h = self.d;
        self.l = self.e;

        self.d = temp_h;
        self.e = temp_l;

        Cycles(4)
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

    fn jz(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        if self.flags.is_zero() {
            self.pc = addr;
        }
        Cycles(10)
    }

    fn jnc(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        if !self.flags.is_carry() {
            self.pc = addr;
        }
        Cycles(10)
    }

    fn jc(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);
        if self.flags.is_carry() {
            self.pc = addr;
        }
        Cycles(10)
    }

    fn jmp(&mut self, mem: &mut Memory) -> Cycles {
        let value = self.fetch_word(mem);
        self.pc = value;
        Cycles(10)
    }

    fn push_b(&mut self, mem: &mut Memory) -> Cycles {
        mem.write_byte(self.sp - 1, self.b);
        mem.write_byte(self.sp - 2, self.c);
        self.sp = self.sp.wrapping_sub(2);
        Cycles(11)
    }

    fn push_d(&mut self, mem: &mut Memory) -> Cycles {
        mem.write_byte(self.sp - 1, self.d);
        mem.write_byte(self.sp - 2, self.e);
        self.sp = self.sp.wrapping_sub(2);
        Cycles(11)
    }

    fn push_h(&mut self, mem: &mut Memory) -> Cycles {
        mem.write_byte(self.sp - 1, self.h);
        mem.write_byte(self.sp - 2, self.l);
        self.sp = self.sp.wrapping_sub(2);
        Cycles(11)
    }

    fn push_psw(&mut self, mem: &mut Memory) -> Cycles {
        mem.write_byte(self.sp - 1, self.a);
        mem.write_byte(self.sp - 2, self.flags.0);
        self.sp = self.sp.wrapping_sub(2);
        Cycles(11)
    }

    fn pop_b(&mut self, mem: &mut Memory) -> Cycles {
        self.c = mem.read_byte(self.sp);
        self.b = mem.read_byte(self.sp + 1);
        self.sp = self.sp.wrapping_add(2);
        Cycles(10)
    }

    fn pop_d(&mut self, mem: &mut Memory) -> Cycles {
        self.e = mem.read_byte(self.sp);
        self.d = mem.read_byte(self.sp + 1);
        self.sp = self.sp.wrapping_add(2);
        Cycles(10)
    }

    fn pop_h(&mut self, mem: &mut Memory) -> Cycles {
        self.l = mem.read_byte(self.sp);
        self.h = mem.read_byte(self.sp + 1);
        self.sp = self.sp.wrapping_add(2);
        Cycles(10)
    }

    fn pop_psw(&mut self, mem: &mut Memory) -> Cycles {
        self.flags.0 = mem.read_byte(self.sp);
        self.a = mem.read_byte(self.sp + 1);
        self.sp = self.sp.wrapping_add(2);
        Cycles(10)
    }

    fn ret(&mut self, mem: &mut Memory) -> Cycles {
        let addr = mem.read_word(self.sp);
        self.sp += 2;
        self.pc = addr;
        Cycles(10)
    }

    fn rz(&mut self, mem: &mut Memory) -> Cycles {
        if self.flags.is_zero() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(10)
        }
    }

    fn rnz(&mut self, mem: &mut Memory) -> Cycles {
        if !self.flags.is_zero() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(10)
        }
    }

    fn rnc(&mut self, mem: &mut Memory) -> Cycles {
        if !self.flags.is_carry() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(10)
        }
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
        let result = (self.a as u16).wrapping_sub(value as u16);
        let c = ((result >> 8) & 0x1) != 0;
        let hc = (!(self.a ^ ((result & 0xFF) as u8) ^ value) & 0x10) != 0;

        self.flags.set((result & 0xFF) as u8, Some(hc), Some(c));
        Cycles(7)
    }

    fn cnz(&mut self, mem: &mut Memory) -> Cycles {
        let addr = self.fetch_word(mem);

        if !self.flags.is_zero() {
            let h = (self.pc >> 8) as u8;
            let l = (self.pc & 0xFF) as u8;

            mem.write_byte(self.sp - 1, h);
            mem.write_byte(self.sp - 2, l);
            self.sp -= 2;
            self.pc = addr;

            Cycles(17)
        } else {
            Cycles(11)
        }
    }

    fn rrc(&mut self, _mem: &mut Memory) -> Cycles {
        let carry = self.a & 0x01;
        self.a = (self.a >> 1) | (carry << 7);
        self.flags.set_carry(carry != 0);
        Cycles(4)
    }

    fn rlc(&mut self, _mem: &mut Memory) -> Cycles {
        let carry = self.a & 0x80 >> 7;
        self.a = (self.a << 1) | carry;
        self.flags.set_carry(carry != 0);
        Cycles(4)
    }

    fn rc(&mut self, mem: &mut Memory) -> Cycles {
        if self.flags.is_carry() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(5)
        }
    }


    fn xra(&mut self, _mem: &mut Memory) -> Cycles {
        self.a = self.a ^ self.a;

        self.flags.set(self.a, Some(false), Some(false));
        Cycles(4)
    }

    fn rp(&mut self, mem: &mut Memory) -> Cycles {
        if !self.flags.is_sign() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(5)
        }
    }

    fn rpo(&mut self, mem: &mut Memory) -> Cycles {
        if !self.flags.is_parity() {
            let addr = mem.read_word(self.sp);
            self.sp += 2;
            self.pc = addr;
            Cycles(11)
        } else {
            Cycles(5)
        }
    }

    fn add_m(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);

        let (res, carry) = self.a.overflowing_add(value);
        let hc = (self.a & 0x0F) + (value & 0x0F) > 0x0F;

        self.flags.set(res, Some(hc), Some(carry));
        self.a = res;

        Cycles(7)
    }

    fn sbb_m(&mut self, mem: &mut Memory) -> Cycles {
        let addr = get_16(self.h, self.l);
        let value = mem.read_byte(addr);

        let value_c = if self.flags.is_carry() { 1 } else { 0 };

        let res_wide = (self.a as u16)
            .wrapping_sub(value as u16)
            .wrapping_sub(value_c);

        let hc = ((self.a & 0x0F) as i16 - (value & 0x0F) as i16 - value_c as i16) < 0;

        let carry_out = (res_wide & 0x100) != 0;

        let res = (res_wide & 0xFF) as u8;
        self.flags.set(res, Some(hc), Some(carry_out));
        self.a = res;

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

