pub struct Stack {
    values: [u16; 12],
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: [0; 12],
        }
    }
    pub fn push(&mut self, sp: u8,  val: u16) {
        self.values[sp as usize] = val;
    }
    pub fn pop(&mut self, sp: u8) -> u16 {
        self.values[sp as usize]
    }
}
