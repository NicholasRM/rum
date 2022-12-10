use crate::ruminfoextr::{get_opcode, get_three_regs, get_one_reg, get_load_reg, get_value, get_two_regs};
use crate::rumcpu::RumCpu;
use crate::rummemory::RumMemory;
use std::io::{Read, self};

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

///
/// A structure that encapsulates our memory segments and our registers in one container
/// 
/// # Members:
/// * `cpu: RumCpu`: stores the registers and the program counter, and it handles register operations
/// * `memory: RumMemory`: stores memory segments and handles segment operations
pub struct RumData {
    pub cpu: RumCpu,
    pub memory: RumMemory,
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::init(program) }
    }

    ///
    /// It starts a loop and grabs a line of instruction at the begginning of each iteration,
    /// the opcode is extracted from the instruction and used to match it to a set of code lines,
    /// which will execute the specified operation, this will continue until it recieves a halt instruction,
    /// or it is given invalid input, in which case it will panic!
    /// 
    pub fn execute(&mut self) {
        let mut instruction;//get it from seg0
        loop{
            instruction = self.memory.get_seg_val(0, self.cpu.pc);
            self.cpu.pc += 1;
            match get_opcode(instruction){
                o if o == Opcode::CMov as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.cmov(reg_a, reg_b, reg_c)
                }
                o if o == Opcode::SegLoad as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    let segment = self.cpu.fetch_reg(reg_b) as usize;
                    let offset = self.cpu.fetch_reg(reg_c) as usize;
                    let value = self.memory.get_seg_val(segment, offset);
                    self.cpu.load_value(reg_a, value);
                }
                o if o == Opcode::SegStore as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    let segment = self.cpu.fetch_reg(reg_a) as usize;
                    let offset = self.cpu.fetch_reg(reg_b) as usize;
                    let value = self.cpu.fetch_reg(reg_c);
                    self.memory.store_seg_val(segment, offset, value);
                }
                o if o == Opcode::Add as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.add(reg_a, reg_b, reg_c)
                }
                o if o == Opcode::Mul as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.mul(reg_a, reg_b, reg_c)
                }
                o if o == Opcode::Div as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.div(reg_a, reg_b, reg_c)
                }
                o if o == Opcode::Nand as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.nand(reg_a, reg_b, reg_c)
                }
                o if o == Opcode::Halt as u32 => {
                    std::process::exit(0);
                }
                o if o == Opcode::Map as u32 => {
                    let (reg_b, reg_c) = get_two_regs(instruction);
                    let size = self.cpu.fetch_reg(reg_c) as usize;
                    let ptr = self.memory.map_seg(size) as u32;
                    self.cpu.load_value(reg_b, ptr);
                }
                o if o == Opcode::Unmap as u32 => {
                    let reg_c = get_one_reg(instruction);
                    let ptr = self.cpu.fetch_reg(reg_c) as usize;
                    self.memory.unmap_seg(ptr);
                }
                o if o == Opcode::Output as u32 => {
                    let reg_c = get_one_reg(instruction);
                    let value = self.cpu.fetch_reg(reg_c);
                    self.output(value);
                }
                o if o == Opcode::Input as u32 => {
                    let reg_c = get_one_reg(instruction);
                    let value = self.input();
                    self.cpu.load_value(reg_c, value)
                }
                o if o == Opcode::LoadProgram as u32 => {
                    let (reg_b, reg_c) = get_two_regs(instruction);
                    let segment = self.cpu.fetch_reg(reg_b) as usize;
                    let offset = self.cpu.fetch_reg(reg_c) as usize;
                    self.memory.load_program(segment, offset);
                    self.cpu.pc = offset;
                }
                o if o == Opcode::LoadValue as u32 => {
                    let load_reg = get_load_reg(instruction);
                    let value = get_value(instruction);
                    self.cpu.load_value(load_reg, value);
                }
                _ => {
                    panic!("invalid instruction");
                }
            }
        }
    }
    
    ///
    /// Takes the value stored in `reg_c` and converts it to an ascii character
    /// If the value stored in `reg_c` is greater than 255 the program will panic!
    /// 
    /// # Arguments:
    /// * `value`: a value equal to reg_c
    pub fn output(&self, value: u32) {
        if value > 255{
            panic!("Value too big");
        }
        print!("{}", char::from_u32(value).unwrap());
    }

    ///
    /// Takes in a character via standard in and returns it back, 
    /// if end of file or control d is signaled, a number filled with ones (maximum values of u32) is returned
    /// 
    pub fn input(&mut self) -> u32 {
        let mut buffer: [u8; 1] = [0];
        let input = io::stdin().read(&mut buffer);
        match input.unwrap(){
            0 => u32::MAX,
            1 => buffer[0] as u32,
            _ => panic!("Too many bites in buffer"),
        }
    }
}
