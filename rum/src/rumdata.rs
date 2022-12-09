use core::panic;

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
            match get_opcode(instruction){
                0 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.cmov(reg_a, reg_b, reg_c)
                }
                1 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    let value = self.memory.get_seg_val(reg_b as u32, reg_c);
                    self.cpu.load_value(reg_a, value);
                }
                2 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    let value = self.cpu.fetch_reg(reg_c);
                    self.memory.store_seg_val(reg_a as u32, reg_b, value);
                }
                3 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.add(reg_a, reg_b, reg_c)
                }
                4 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.mul(reg_a, reg_b, reg_c)
                }
                5 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.div(reg_a, reg_b, reg_c)
                }
                6 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.nand(reg_a, reg_b, reg_c)
                }
                7 => {
                    std::process::exit(0);
                }
                8 => {
                    let (reg_b, reg_c) = get_two_regs(instruction);
                    self.memory.map(reg_b as u32, reg_c);
                }
                9 => {
                    let reg_c = get_one_reg(instruction);
                    self.memory.unmap(reg_c as u32);
                }
                10 => {
                    let reg_c = get_one_reg(instruction);
                    self.output(reg_c);
                }
                11 => {
                    let reg_c = get_one_reg(instruction);
                    self.input(reg_c);
                }
                12 => {
                    let (reg_b, reg_c) = get_two_regs(instruction);
                    self.memory.load_program(reg_b as u32, reg_c);
                    self.cpu.pc = reg_c;
                }
                13 => {
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
    /// If the the value stored in `reg_c` is greater than 255 the program will panic!
    /// 
    /// # Arguments:
    /// * `reg_c`: an index to an array, it represents a register address
    pub fn output(&self, reg_c: usize) {
        if self.cpu.regs[reg_c] > 255{
            panic!("Value too big");
        }
        print!("{}", self.cpu.regs[reg_c] as u8 as char);
    }

    ///
    /// Takes in a character via standard in and stores it in `reg_c`, 
    /// if end of file or control d is signaled, `reg_c` is filled with ones (maximum values of u32)
    /// 
    /// # Arguments: 
    /// * `reg_c`: an index to an array, it represents a register address
    /// 
    pub fn input(&mut self, reg_c: usize) {
        let mut buffer: [u8; 1] = [0];
        let input = io::stdin().read(&mut buffer);
        match input.unwrap(){
            0 => self.cpu.load_value(reg_c, u32::MAX),
            1 => self.cpu.load_value(reg_c, buffer[0] as u32),
            _ => panic!("Too many bytes in buffer"),
        }
    }
}
