pub struct RumCpu {
    pc: usize,
    regs: [u32; 8],
}

impl RumCpu {
    pub fn init() -> Self {
        RumCpu { pc: 0, regs: [0; 8] }
    }

    pub fn cmov(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        if self.regs[reg_c] != 0{
            self.regs[reg_a] = self.regs[reg_b];
        }
    }
    pub fn add(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        let temp_b: u64 = self.regs[reg_b] as u64;
        let temp_c: u64 = self.regs[reg_c] as u64;
        self.regs[reg_a] = ((temp_b + temp_c) % (u32::MAX) as u64) as u32;
    }
    pub fn mul(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        let temp_b: u64 = self.regs[reg_b] as u64;
        let temp_c: u64 = self.regs[reg_c] as u64;
        self.regs[reg_a] = ((temp_b * temp_c) % (u32::MAX) as u64) as u32;
    }
    pub fn div(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.regs[reg_a] = self.regs[reg_b] / self.regs[reg_c];
    }
    pub fn nand(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.regs[reg_a] = !(self.regs[reg_b] & self.regs[reg_c]);
    }
    pub fn output() {todo!()}
    pub fn input() {todo!()}
    pub fn load_value(&mut self, load_reg: usize, value: u32) {
        self.regs[load_reg] = value;
    }
}