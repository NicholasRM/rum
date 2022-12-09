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

pub struct RumData {
    pub cpu: RumCpu,
    pub memory: RumMemory,
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::init(program) }
    }

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
                    let value = self.memory.get_seg_val(reg_b as u32, reg_c);
                    self.cpu.load_value(reg_a, value);
                }
                o if o == Opcode::SegStore as u32 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    let value = self.cpu.fetch_reg(reg_c);
                    self.memory.store_seg_val(reg_a as u32, reg_b, value);
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
                    self.memory.map_seg(reg_b as u32, reg_c);
                }
                o if o == Opcode::Unmap as u32 => {
                    let reg_c = get_one_reg(instruction);
                    self.memory.unmap_seg(reg_c as u32);
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
                    self.memory.load_program(reg_b as u32, reg_c);
                    self.cpu.pc = reg_c;
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
    
    pub fn output(&self, value: u32) {
        if value > 255{
            panic!("Value too big");
        }
        print!("{}", char::from_u32(value).unwrap());
    }

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
