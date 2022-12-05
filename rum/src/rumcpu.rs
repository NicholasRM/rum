pub struct RumCpu {
    pc: usize,
    regs: [u32; 8],
}

impl RumCpu {
    pub fn init() -> Self {
        RumCpu { pc: 0, regs: [0; 8] }
    }

    pub fn cmov() {todo!()}
    pub fn add() {todo!()}
    pub fn mul() {todo!()}
    pub fn div() {todo!()}
    pub fn nand() {todo!()}
    pub fn output() {todo!()}
    pub fn input() {todo!()}
    pub fn load_value() {todo!()}
}