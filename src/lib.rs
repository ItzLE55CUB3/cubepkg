#[derive(Debug)]
pub struct Operation {
    opcode: u8,
    opt1: i32,
    opt2: i32,
}

impl Operation {
    pub fn new(opcode: u8, opt1: i32, opt2: i32) -> Self {
        Self {
            opcode,
            opt1,
            opt2
        }
    }

    pub fn generate(&self) -> Vec<u8> {
        if self.opcode <= 1 {
            // TODO: 加紧修理这坨💩
            // NOTE: 给人类9178年都写不出这种代码
            return vec![0x32, self.opcode << 4 | self.opt1 as u8, 
                    (self.opt2 << 24 >> 24) as u8,
                    (self.opt2 >> 8 << 24 >> 24) as u8,
                    (self.opt2 >> 16 << 24 >> 24) as u8,
                    (self.opt2 >> 24) as u8];
        } else if self.opcode <= 2 {
            return vec![0xFF, self.opcode << 4 | self.opt1 as u8,
                self.opt2 as u8]
        } 
        vec![]
    }
}