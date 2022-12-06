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
    cpu: RumCpu,
    memory: RumMemory,
    seg0: Vec<u32>,
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::new(), seg0: program }
    }

    pub fn execute(&mut self) {
        todo!();
    }
}
