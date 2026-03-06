
const FLAG_SIGN: u8 = 0b10000000;
const FLAG_ZERO: u8 = 0b01000000;
const FLAG_AUX_CARRY: u8 = 0b00010000;
const FLAG_PARITY: u8 = 0b00000100;
const FLAG_CARRY: u8 = 0b00000001;

pub struct Flags(pub u8);

impl Flags {
    pub fn is_zero(&self) -> bool {
        self.0 & FLAG_ZERO != 0
    }

    pub fn is_carry(&self) -> bool {
        self.0 & FLAG_CARRY != 0
    }

    pub fn is_half_carry(&self) -> bool {
        self.0 & FLAG_AUX_CARRY != 0
    }

    pub fn is_parity(&self) -> bool {
        self.0 & FLAG_PARITY != 0
    }

    pub fn is_sign(&self) -> bool {
        self.0 & FLAG_SIGN != 0
    }

    pub fn get_carry(&self, v1: u8, v2: u8) -> bool {
        v1 < v2
    }

    pub fn get_half_carry(&self, v1: u8, v2: u8) -> bool {
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
        // Even parity: true if number of set bits is even
        let p = value.count_ones() % 2 == 0;

        let mut new_flags = 0u8;
        if s { new_flags |= FLAG_SIGN; }
        if z { new_flags |= FLAG_ZERO; }
        if p { new_flags |= FLAG_PARITY; }

        // Bit 1 is ALWAYS 1 on the 8080
        new_flags |= 0b00000010;

        // Handle AC
        if let Some(hc) = half_carry {
            if hc { new_flags |= FLAG_AUX_CARRY; }
        } else {
            new_flags |= self.0 & FLAG_AUX_CARRY;
        }

        // Handle CY
        if let Some(c) = carry {
            if c { new_flags |= FLAG_CARRY; }
        } else {
            new_flags |= self.0 & FLAG_CARRY;
        }

        self.0 = new_flags;
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.0 = (self.0 & !1) | (carry as u8);
    }
}
