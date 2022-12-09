pub struct RumCpu {
    pub pc: usize,
    pub regs: [u32; 8],
}

impl RumCpu {
    pub fn init() -> Self {
        RumCpu { pc: 0, regs: [0; 8] }
    }

    ///
    /// Sets the register `reg_a` to the value of `reg_b` if the value in `reg_c` is not 0
    /// 
    ///  # Arguments:
    /// * `reg_a`: an index to an array, it represents a register address
    /// * `reg_b`: an index to an array, it represents a register address
    /// * `reg_c`: an index to an array, it represents a register address
    /// 
    pub fn cmov(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        if self.regs[reg_c] != 0{
            self.regs[reg_a] = self.regs[reg_b];
        }
    }
    
    ///
    /// Adds the values in `reg_b` and `reg_c`, then executes bitwise AND on the sum and the maximum value of a u32
    /// to it to wrap around incase of overflow, the result is then stored in `reg_a`
    ///
    /// # Arguments:
    /// * `reg_a`: an index to an array, it represents a register address
    /// * `reg_b`: an index to an array, it represents a register address
    /// * `reg_c`: an index to an array, it represents a register address
    /// 
    pub fn add(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        let temp_b: u64 = self.regs[reg_b] as u64;
        let temp_c: u64 = self.regs[reg_c] as u64;
        self.regs[reg_a] = ((temp_b + temp_c) & (u32::MAX) as u64) as u32;
    }

    ///
    /// Multiplies the values in `reg_b` and `reg_c`, then executes bitwise AND on the product and the maximum value of a u32
    /// to it to wrap around incase of overflow, the result is then stored in `reg_a`
    ///
    /// # Arguments:
    /// * `reg_a`: an index to an array, it represents a register address
    /// * `reg_b`: an index to an array, it represents a register address
    /// * `reg_c`: an index to an array, it represents a register address
    ///
    pub fn mul(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        let temp_b: u64 = self.regs[reg_b] as u64;
        let temp_c: u64 = self.regs[reg_c] as u64;
        self.regs[reg_a] = ((temp_b * temp_c) & (u32::MAX) as u64) as u32;
    }

    ///
    /// Divides the value in reg_b by the value in reg_c and stores the result in reg_a rounded down,
    /// if reg_c contians a 0, the program will panic! before the division takes place
    /// 
    /// # Arguments:
    /// * `reg_a`: an index to an array, it represents a register address
    /// * `reg_b`: an index to an array, it represents a register address
    /// * `reg_c`: an index to an array, it represents a register address
    ///
    pub fn div(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.regs[reg_a] = self.regs[reg_b] / self.regs[reg_c];
    }

    ///
    /// Performs bitwise NAND on the values in `reg_b` and `reg_c` and stores the result in `reg_a`
    /// 
    /// # Arguments:
    /// * `reg_a`: an index to an array, it represents a register address
    /// * `reg_b`: an index to an array, it represents a register address
    /// * `reg_c`: an index to an array, it represents a register address
    ///
    pub fn nand(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.regs[reg_a] = !(self.regs[reg_b] & self.regs[reg_c]);
    }

    ///
    /// Loads `value` into the `load_reg`
    /// 
    /// # Arguments:
    /// * `load_reg`: an index to an array, it represents a register address
    /// * `value`: a u32 value
    /// 
    pub fn load_value(&mut self, load_reg: usize, value: u32) {
        self.regs[load_reg] = value;
    }

    pub fn fetch_reg(&self, address: usize) -> u32{
        self.regs[address]
    }
}