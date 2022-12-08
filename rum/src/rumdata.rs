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
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::init(program) }
    }

    pub fn execute(&mut self) {
        todo!();
    }
    
    fn output() {todo!()}
    fn input() {todo!()}
}
