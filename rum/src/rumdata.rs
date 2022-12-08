use core::panic;

use crate::rumcpu::RumCpu;
use crate::rummemory::RumMemory;

enum Opcode {
    CMov,
    SegLoad,
    SegStore,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    Map,
    Unmap,
    Output,
    Input,
    LoadProgram,
    LoadValue
}

pub struct RumData {
    pub cpu: RumCpu,
    pub memory: RumMemory,
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::init(program) }
    }

    pub fn execute(&mut self) {
        todo!();
    }
    
    pub fn output(&self, reg_c: usize) {
        if self.cpu.regs[reg_c] > 255{
            panic!("Value too big");
        }
    }
    fn input() {todo!()}
}
